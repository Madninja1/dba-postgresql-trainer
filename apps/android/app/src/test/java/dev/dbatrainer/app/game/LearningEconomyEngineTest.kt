package dev.dbatrainer.app.game

import dev.dbatrainer.app.game.domain.ContentAccessState
import dev.dbatrainer.app.game.domain.KnowledgeTopicProgress
import dev.dbatrainer.app.game.domain.LearningProductType
import dev.dbatrainer.app.game.domain.newDbaWorldState
import dev.dbatrainer.app.game.engine.LearningEconomyEngine
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class LearningEconomyEngineTest {
    @Test
    fun starterTopicsUseApprovedFreeRules() {
        assertTrue(LearningEconomyEngine.isFreeTopic("linux-dba-1", 5))
        assertTrue(LearningEconomyEngine.isFreeTopic("dba-1", 3))
        assertFalse(LearningEconomyEngine.isFreeTopic("dba-1", 4))
        assertTrue(LearningEconomyEngine.isFreeTopic("sql-1", 2))
        assertFalse(LearningEconomyEngine.isFreeTopic("sql-1", 3))
        assertTrue(LearningEconomyEngine.isFreeTopic("psql-1", 2))
        assertFalse(LearningEconomyEngine.isFreeTopic("psql-1", 3))
        assertTrue(LearningEconomyEngine.isFreeTopic("plpgsql-1", 1))
        assertFalse(LearningEconomyEngine.isFreeTopic("plpgsql-1", 2))
    }

    @Test
    fun firstPaidDbaTopicCostsCurrentIncidentEarnings() {
        val prices = LearningEconomyEngine.prices("dba-1", 4)
        assertEquals(450, prices.bundle)
    }

    @Test
    fun bundlePurchaseDeductsCreditsAndUnlocksTopicOnce() {
        val progress = topic("dba-1", "dba1-arch-general", 4)
        val funded = newDbaWorldState().copy(
            player = newDbaWorldState().player.copy(credits = 450),
        )

        val first = LearningEconomyEngine.purchaseBundle(funded, progress)
        assertTrue(first.purchased)
        assertEquals(0, first.world.player.credits)
        assertTrue("dba-1::dba1-arch-general" in first.world.purchasedTopicBundles)
        assertEquals(
            ContentAccessState.PURCHASED,
            LearningEconomyEngine.offerFor(progress, first.world).accessState,
        )

        val repeated = LearningEconomyEngine.purchaseBundle(first.world, progress)
        assertFalse(repeated.purchased)
        assertEquals(0, repeated.world.player.credits)
    }


    @Test
    fun individualPurchaseUnlocksOnlySelectedProductAndWritesHistory() {
        val progress = topic("dba-1", "dba1-arch-general", 4)
        val funded = newDbaWorldState().copy(
            player = newDbaWorldState().player.copy(credits = 500),
        )

        val purchase = LearningEconomyEngine.purchaseProduct(
            funded,
            progress,
            LearningProductType.NOTES,
        )
        val offer = LearningEconomyEngine.offerFor(progress, purchase.world)

        assertTrue(purchase.purchased)
        assertEquals(ContentAccessState.PURCHASED, offer.notesAccess)
        assertEquals(ContentAccessState.LOCKED, offer.testAccess)
        assertEquals(1, purchase.world.learningTransactions.size)
        assertEquals(LearningProductType.NOTES, purchase.world.learningTransactions.single().productType)
        assertEquals(purchase.world.player.credits, purchase.world.learningTransactions.single().balanceAfter)
    }

    @Test
    fun bundleUpgradeDoesNotChargePurchasedMaterialTwice() {
        val progress = topic("dba-1", "dba1-arch-general", 4)
        val funded = newDbaWorldState().copy(
            player = newDbaWorldState().player.copy(credits = 700),
        )
        val notes = LearningEconomyEngine.purchaseProduct(
            funded,
            progress,
            LearningProductType.NOTES,
        )
        val afterNotes = LearningEconomyEngine.offerFor(progress, notes.world)
        val expectedUpgrade = afterNotes.prices.bundle - afterNotes.prices.notes

        val bundle = LearningEconomyEngine.purchaseBundle(notes.world, progress)

        assertTrue(bundle.purchased)
        assertEquals(700 - afterNotes.prices.notes - expectedUpgrade, bundle.world.player.credits)
        assertTrue(LearningEconomyEngine.offerFor(progress, bundle.world).allAvailableProductsAccessible)
        assertEquals(2, bundle.world.learningTransactions.size)
    }

    @Test
    fun labCannotBePurchasedWhenTopicHasNoLab() {
        val progress = topic("dba-1", "dba1-arch-general", 4)
        val funded = newDbaWorldState().copy(
            player = newDbaWorldState().player.copy(credits = 1_000),
        )

        val result = LearningEconomyEngine.purchaseProduct(
            funded,
            progress,
            LearningProductType.LAB,
        )

        assertFalse(result.purchased)
        assertEquals(1_000, result.world.player.credits)
        assertTrue(result.world.learningTransactions.isEmpty())
    }

    @Test
    fun masteredTopicNeverBecomesLocked() {
        val mastered = topic("dba-1", "dba1-arch-general", 4).copy(
            started = true,
            passed = true,
            completedAttempts = 1,
            bestAccuracyPercent = 90.0,
        )

        assertEquals(
            ContentAccessState.MASTERED,
            LearningEconomyEngine.offerFor(mastered, newDbaWorldState()).accessState,
        )
    }

    private fun topic(
        courseCode: String,
        slug: String,
        topicNumber: Long,
    ) = KnowledgeTopicProgress(
        courseCode = courseCode,
        topicSlug = slug,
        topicNumber = topicNumber,
        title = slug,
        started = false,
        passed = false,
        completedAttempts = 0,
        bestAccuracyPercent = 0.0,
    )
}
