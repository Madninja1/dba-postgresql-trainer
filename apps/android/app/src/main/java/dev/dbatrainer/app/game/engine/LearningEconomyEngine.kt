package dev.dbatrainer.app.game.engine

import dev.dbatrainer.app.game.domain.ContentAccessState
import dev.dbatrainer.app.game.domain.KnowledgeSnapshot
import dev.dbatrainer.app.game.domain.KnowledgeTopicProgress
import dev.dbatrainer.app.game.domain.LearningProductPrice
import dev.dbatrainer.app.game.domain.LearningProductType
import dev.dbatrainer.app.game.domain.LearningPurchaseResult
import dev.dbatrainer.app.game.domain.LearningTopicOffer
import dev.dbatrainer.app.game.domain.LearningTransaction
import dev.dbatrainer.app.game.domain.WorldState
import dev.dbatrainer.app.game.domain.learningProductKey

object LearningEconomyEngine {
    private val supportedCourses = setOf(
        "linux-dba-1",
        "dba-1",
        "sql-1",
        "psql-1",
        "plpgsql-1",
    )

    fun offers(
        snapshot: KnowledgeSnapshot,
        world: WorldState,
    ): List<LearningTopicOffer> = snapshot.topics
        .values
        .asSequence()
        .filter { it.courseCode in supportedCourses }
        .sortedWith(compareBy<KnowledgeTopicProgress> { courseOrder(it.courseCode) }.thenBy { it.topicNumber })
        .map { progress -> offerFor(progress, world) }
        .toList()

    fun offerFor(
        progress: KnowledgeTopicProgress,
        world: WorldState,
    ): LearningTopicOffer {
        val topicKey = KnowledgeSnapshot.topicKey(progress.courseCode, progress.topicSlug)
        val free = isFreeTopic(progress.courseCode, progress.topicNumber)
        val bundlePurchased = topicKey in world.purchasedTopicBundles ||
            learningProductKey(topicKey, LearningProductType.BUNDLE) in world.purchasedLearningProducts
        val prices = prices(progress.courseCode, progress.topicNumber, progress.hasLab)
        val notesAccess = productAccess(progress, world, LearningProductType.NOTES, free, bundlePurchased)
        val testAccess = productAccess(progress, world, LearningProductType.TEST, free, bundlePurchased)
        val labAccess = if (progress.hasLab) {
            productAccess(progress, world, LearningProductType.LAB, free, bundlePurchased)
        } else {
            null
        }
        val individuallyPurchasedAmount = listOf(
            LearningProductType.NOTES to prices.notes,
            LearningProductType.TEST to prices.test,
            LearningProductType.LAB to prices.lab,
        ).sumOf { (type, price) ->
            if (type == LearningProductType.LAB && !progress.hasLab) {
                0
            } else if (isIndividuallyPurchased(world, topicKey, type)) {
                price
            } else {
                0
            }
        }
        val aggregateState = when {
            progress.passed -> ContentAccessState.MASTERED
            free -> ContentAccessState.FREE
            bundlePurchased || individuallyPurchasedAmount > 0 -> ContentAccessState.PURCHASED
            else -> ContentAccessState.LOCKED
        }

        return LearningTopicOffer(
            courseCode = progress.courseCode,
            topicSlug = progress.topicSlug,
            topicNumber = progress.topicNumber,
            title = progress.title,
            accessState = aggregateState,
            prices = prices,
            hasLab = progress.hasLab,
            notesAccess = notesAccess,
            testAccess = testAccess,
            labAccess = labAccess,
            bundlePurchased = bundlePurchased,
            bundleUpgradePrice = if (free || progress.passed || bundlePurchased) {
                0
            } else {
                (prices.bundle - individuallyPurchasedAmount).coerceAtLeast(0)
            },
        )
    }

    fun purchaseProduct(
        world: WorldState,
        progress: KnowledgeTopicProgress,
        productType: LearningProductType,
    ): LearningPurchaseResult {
        val offer = offerFor(progress, world)
        if (productType == LearningProductType.LAB && !offer.hasLab) {
            return result(
                world = world,
                purchased = false,
                ru = "Для этой темы лабораторная пока не предусмотрена.",
                en = "No lab is available for this topic yet.",
            )
        }

        if (productType == LearningProductType.BUNDLE) {
            if (offer.accessState == ContentAccessState.FREE ||
                offer.accessState == ContentAccessState.MASTERED ||
                offer.bundlePurchased ||
                offer.allAvailableProductsAccessible
            ) {
                return alreadyAvailable(world)
            }
            return purchase(
                world = world,
                progress = progress,
                productType = productType,
                price = offer.bundleUpgradePrice,
            )
        }

        if (accessForType(offer, productType) != ContentAccessState.LOCKED) {
            return alreadyAvailable(world)
        }

        return purchase(
            world = world,
            progress = progress,
            productType = productType,
            price = offer.prices.forType(productType),
        )
    }

    fun purchaseBundle(
        world: WorldState,
        progress: KnowledgeTopicProgress,
    ): LearningPurchaseResult = purchaseProduct(world, progress, LearningProductType.BUNDLE)

    fun canAccessProduct(
        world: WorldState,
        progress: KnowledgeTopicProgress,
        productType: LearningProductType,
    ): Boolean {
        if (productType == LearningProductType.LAB && !progress.hasLab) {
            return false
        }
        return accessForType(offerFor(progress, world), productType) != ContentAccessState.LOCKED
    }

    fun hasPurchasedAccess(
        world: WorldState,
        progress: KnowledgeTopicProgress,
    ): Boolean {
        val topicKey = KnowledgeSnapshot.topicKey(progress.courseCode, progress.topicSlug)
        return topicKey in world.purchasedTopicBundles ||
            LearningProductType.entries.any { type ->
                learningProductKey(topicKey, type) in world.purchasedLearningProducts
            }
    }

    fun isFreeTopic(courseCode: String, topicNumber: Long): Boolean = when (courseCode) {
        "linux-dba-1" -> true
        "dba-1" -> topicNumber <= 3L
        "sql-1", "psql-1" -> topicNumber <= 2L
        "plpgsql-1" -> topicNumber <= 1L
        else -> false
    }

    fun prices(
        courseCode: String,
        topicNumber: Long,
        hasLab: Boolean = true,
    ): LearningProductPrice {
        if (isFreeTopic(courseCode, topicNumber)) {
            return LearningProductPrice(notes = 0, test = 0, lab = 0, bundle = 0)
        }

        val bundle = when (courseCode) {
            "dba-1" -> 450 + ((topicNumber - 4L).coerceAtLeast(0L).toInt() * 50)
            "sql-1" -> 500
            "psql-1" -> 450
            "plpgsql-1" -> 450 + ((topicNumber - 2L).coerceAtLeast(0L).toInt() * 150)
            else -> 500
        }
        return LearningProductPrice(
            notes = roundToTen(bundle * 55 / 100),
            test = roundToTen(bundle * 50 / 100),
            lab = if (hasLab) roundToTen(bundle * 45 / 100) else 0,
            bundle = bundle,
        )
    }

    private fun purchase(
        world: WorldState,
        progress: KnowledgeTopicProgress,
        productType: LearningProductType,
        price: Int,
    ): LearningPurchaseResult {
        if (world.player.credits < price) {
            return result(
                world = world,
                purchased = false,
                ru = "Недостаточно Engineer Credits: нужно $price CR, доступно ${world.player.credits} CR.",
                en = "Not enough Engineer Credits: $price CR required, ${world.player.credits} CR available.",
            )
        }

        val topicKey = KnowledgeSnapshot.topicKey(progress.courseCode, progress.topicSlug)
        val productKey = learningProductKey(topicKey, productType)
        val balanceAfter = world.player.credits - price
        val transaction = LearningTransaction(
            id = (world.learningTransactions.maxOfOrNull { it.id } ?: 0L) + 1L,
            topicKey = topicKey,
            productType = productType,
            price = price,
            balanceAfter = balanceAfter,
            worldTotalMinutes = world.clock.totalMinutes(),
        )
        val updated = world.copy(
            player = world.player.copy(credits = balanceAfter),
            purchasedTopicBundles = if (productType == LearningProductType.BUNDLE) {
                world.purchasedTopicBundles + topicKey
            } else {
                world.purchasedTopicBundles
            },
            purchasedLearningProducts = world.purchasedLearningProducts + productKey,
            learningTransactions = world.learningTransactions + transaction,
        )
        val productRu = productLabel(productType, true)
        val productEn = productLabel(productType, false)
        return result(
            world = updated,
            purchased = true,
            ru = "$productRu «${progress.title}» открыт за $price CR.",
            en = "$productEn for ${progress.title} unlocked for $price CR.",
        )
    }

    private fun productAccess(
        progress: KnowledgeTopicProgress,
        world: WorldState,
        productType: LearningProductType,
        free: Boolean,
        bundlePurchased: Boolean,
    ): ContentAccessState = when {
        progress.passed -> ContentAccessState.MASTERED
        free -> ContentAccessState.FREE
        bundlePurchased -> ContentAccessState.PURCHASED
        isIndividuallyPurchased(
            world,
            KnowledgeSnapshot.topicKey(progress.courseCode, progress.topicSlug),
            productType,
        ) -> ContentAccessState.PURCHASED
        else -> ContentAccessState.LOCKED
    }

    private fun isIndividuallyPurchased(
        world: WorldState,
        topicKey: String,
        productType: LearningProductType,
    ): Boolean = learningProductKey(topicKey, productType) in world.purchasedLearningProducts

    private fun accessForType(
        offer: LearningTopicOffer,
        productType: LearningProductType,
    ): ContentAccessState = when (productType) {
        LearningProductType.NOTES -> offer.notesAccess
        LearningProductType.TEST -> offer.testAccess
        LearningProductType.LAB -> offer.labAccess ?: ContentAccessState.LOCKED
        LearningProductType.BUNDLE -> offer.accessState
    }

    private fun alreadyAvailable(world: WorldState): LearningPurchaseResult = result(
        world = world,
        purchased = false,
        ru = "Этот учебный материал уже доступен.",
        en = "This learning content is already available.",
    )

    private fun result(
        world: WorldState,
        purchased: Boolean,
        ru: String,
        en: String,
    ) = LearningPurchaseResult(
        world = world,
        purchased = purchased,
        messageRu = ru,
        messageEn = en,
    )

    private fun productLabel(type: LearningProductType, russian: Boolean): String = when (type) {
        LearningProductType.NOTES -> if (russian) "Конспект" else "Notes"
        LearningProductType.TEST -> if (russian) "Тест" else "Test"
        LearningProductType.LAB -> if (russian) "Лабораторная" else "Lab"
        LearningProductType.BUNDLE -> if (russian) "Полный блок" else "Full bundle"
    }

    private fun roundToTen(value: Int): Int = ((value + 9) / 10) * 10

    private fun courseOrder(courseCode: String): Int = when (courseCode) {
        "linux-dba-1" -> 0
        "dba-1" -> 1
        "sql-1" -> 2
        "psql-1" -> 3
        "plpgsql-1" -> 4
        else -> 100
    }
}
