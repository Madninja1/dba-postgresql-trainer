package dev.dbatrainer.app.game.domain

enum class ContentAccessState {
    FREE,
    LOCKED,
    PURCHASED,
    MASTERED,
}

enum class LearningProductType {
    NOTES,
    TEST,
    LAB,
    BUNDLE,
}

data class LearningProductPrice(
    val notes: Int,
    val test: Int,
    val lab: Int,
    val bundle: Int,
) {
    val separateTotal: Int
        get() = notes + test + lab

    val bundleDiscount: Int
        get() = (separateTotal - bundle).coerceAtLeast(0)

    fun forType(type: LearningProductType): Int = when (type) {
        LearningProductType.NOTES -> notes
        LearningProductType.TEST -> test
        LearningProductType.LAB -> lab
        LearningProductType.BUNDLE -> bundle
    }
}

data class LearningTopicOffer(
    val courseCode: String,
    val topicSlug: String,
    val topicNumber: Long,
    val title: String,
    val accessState: ContentAccessState,
    val prices: LearningProductPrice,
    val hasLab: Boolean,
    val notesAccess: ContentAccessState,
    val testAccess: ContentAccessState,
    val labAccess: ContentAccessState?,
    val bundlePurchased: Boolean,
    val bundleUpgradePrice: Int,
) {
    val allAvailableProductsAccessible: Boolean
        get() = notesAccess != ContentAccessState.LOCKED &&
            testAccess != ContentAccessState.LOCKED &&
            (labAccess == null || labAccess != ContentAccessState.LOCKED)
}

data class LearningTransaction(
    val id: Long,
    val topicKey: String,
    val productType: LearningProductType,
    val price: Int,
    val balanceAfter: Int,
    val worldTotalMinutes: Int,
)

data class LearningPurchaseResult(
    val world: WorldState,
    val purchased: Boolean,
    val messageRu: String,
    val messageEn: String,
)

fun learningProductKey(
    courseCode: String,
    topicSlug: String,
    productType: LearningProductType,
): String = learningProductKey(
    topicKey = KnowledgeSnapshot.topicKey(courseCode, topicSlug),
    productType = productType,
)

fun learningProductKey(
    topicKey: String,
    productType: LearningProductType,
): String = "$topicKey::${productType.name}"
