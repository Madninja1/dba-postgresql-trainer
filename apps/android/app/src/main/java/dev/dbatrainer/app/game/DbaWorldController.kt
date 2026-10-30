package dev.dbatrainer.app.game

import android.content.Context
import android.os.SystemClock
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateListOf
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.setValue
import dev.dbatrainer.app.game.data.WorldRepository
import dev.dbatrainer.app.game.domain.DailyTaskKind
import dev.dbatrainer.app.game.domain.IncidentKnowledgeAccess
import dev.dbatrainer.app.game.domain.IncidentStatus
import dev.dbatrainer.app.game.domain.InfrastructureNode
import dev.dbatrainer.app.game.domain.KnowledgeMode
import dev.dbatrainer.app.game.domain.LearningProductType
import dev.dbatrainer.app.game.domain.LearningTopicOffer
import dev.dbatrainer.app.game.domain.KnowledgeSnapshot
import dev.dbatrainer.app.game.domain.WorldState
import dev.dbatrainer.app.game.domain.newDbaWorldState
import dev.dbatrainer.app.game.engine.IncidentEngine
import dev.dbatrainer.app.game.engine.KnowledgeEngine
import dev.dbatrainer.app.game.engine.LearningEconomyEngine
import dev.dbatrainer.app.game.engine.ProgressionEngine
import dev.dbatrainer.app.game.engine.TerminalEngine
import dev.dbatrainer.app.game.engine.WorldClockEngine
import dev.dbatrainer.app.game.engine.WorkdayEngine

enum class DbaWorldScreen {
    Landing,
    Outdoor,
    Workstation,
    Computer,
    Monitoring,
    World,
    NodeDetails,
    Terminal,
    LearningCenter,
    LearningHistory,
}

data class TerminalLine(
    val command: String?,
    val output: String,
)

class DbaWorldController(context: Context) {
    private val repository = WorldRepository(context)
    private var activeSinceRealtimeMillis: Long? = null
    private var trackedWorld: WorldState? = null
    private var terminalReturnScreen = DbaWorldScreen.Computer
    private var learningCenterReturnScreen = DbaWorldScreen.World
    private var nodeDetailsReturnScreen = DbaWorldScreen.World

    var screen by mutableStateOf(DbaWorldScreen.Landing)
        private set

    var world by mutableStateOf<WorldState?>(null)
        private set

    var latestHint by mutableStateOf<String?>(null)
        private set

    var knowledgeSnapshot by mutableStateOf(KnowledgeSnapshot.EMPTY)
        private set

    var learningMessage by mutableStateOf<String?>(null)
        private set

    var selectedNodeId by mutableStateOf<String?>(null)
        private set

    val selectedNode: InfrastructureNode?
        get() = world?.nodes?.firstOrNull { it.id == selectedNodeId }

    val terminalHistory = mutableStateListOf<TerminalLine>()

    val learningOffers: List<LearningTopicOffer>
        get() = world?.let { current -> LearningEconomyEngine.offers(knowledgeSnapshot, current) }.orEmpty()

    val activeKnowledgeAccess: IncidentKnowledgeAccess?
        get() {
            val currentWorld = world ?: return null
            val kind = currentWorld.incident?.kind ?: return null
            return KnowledgeEngine.accessFor(kind, knowledgeSnapshot, currentWorld)
        }

    val terminalSuggestions: List<String>
        get() = if (activeKnowledgeAccess?.mode == KnowledgeMode.GUIDED) {
            world?.let(TerminalEngine::suggestedCommands).orEmpty()
        } else {
            emptyList()
        }

    fun guidedTip(russian: Boolean): String? {
        val access = activeKnowledgeAccess ?: return null
        if (access.mode != KnowledgeMode.GUIDED) {
            return null
        }
        return KnowledgeEngine.guidedTip(access.incidentKind, russian)
    }

    val canContinue: Boolean
        get() = repository.hasSavedWorld()

    fun open(snapshot: KnowledgeSnapshot) {
        knowledgeSnapshot = snapshot
        world = null
        latestHint = null
        learningMessage = null
        selectedNodeId = null
        terminalHistory.clear()
        screen = DbaWorldScreen.Landing
    }

    fun updateKnowledge(snapshot: KnowledgeSnapshot) {
        knowledgeSnapshot = snapshot
    }

    fun newGame() {
        activeSinceRealtimeMillis = SystemClock.elapsedRealtime()
        val created = newDbaWorldState()
        repository.save(created)
        trackedWorld = created
        world = created
        latestHint = null
        learningMessage = null
        selectedNodeId = null
        terminalHistory.clear()
        screen = DbaWorldScreen.Outdoor
    }

    fun continueGame() {
        val loaded = repository.load()
        if (loaded == null) {
            newGame()
            return
        }

        val upgraded = IncidentEngine.ensureActiveIncident(loaded)
        repository.save(upgraded)
        trackedWorld = upgraded
        world = upgraded
        latestHint = null
        learningMessage = null
        selectedNodeId = null
        terminalHistory.clear()
        screen = DbaWorldScreen.Outdoor
    }

    fun enterBuilding() {
        if (world == null) {
            return
        }
        screen = DbaWorldScreen.Workstation
    }

    fun goOutdoor() {
        if (world == null) {
            return
        }
        screen = DbaWorldScreen.Outdoor
    }

    fun openWorkstation() {
        if (world == null) {
            return
        }
        screen = DbaWorldScreen.Workstation
    }

    fun openComputer() {
        if (world == null) {
            return
        }
        screen = DbaWorldScreen.Computer
    }

    fun openMonitoring() {
        val current = world ?: return
        val updated = WorkdayEngine.completeTask(current, DailyTaskKind.REVIEW_MONITORING)
        world = updated
        trackedWorld = updated
        repository.save(updated)
        screen = DbaWorldScreen.Monitoring
    }

    fun openServerRoom() {
        val current = world ?: return
        val updated = WorkdayEngine.completeTask(current, DailyTaskKind.INSPECT_SERVER_ROOM)
        world = updated
        trackedWorld = updated
        repository.save(updated)
        screen = DbaWorldScreen.World
    }

    fun closeShift() {
        val current = world ?: return
        val updated = WorkdayEngine.closeShift(current)
        world = updated
        trackedWorld = updated
        repository.save(updated)
    }

    fun openNodeDetails(nodeId: String) {
        val current = world ?: return
        if (current.nodes.none { it.id == nodeId }) {
            return
        }
        nodeDetailsReturnScreen = if (screen == DbaWorldScreen.Monitoring) {
            DbaWorldScreen.Monitoring
        } else {
            DbaWorldScreen.World
        }
        selectedNodeId = nodeId
        screen = DbaWorldScreen.NodeDetails
    }

    fun closeNodeDetails() {
        selectedNodeId = null
        screen = nodeDetailsReturnScreen
    }

    fun openLearningCenter() {
        if (world == null) {
            return
        }
        learningCenterReturnScreen = when (screen) {
            DbaWorldScreen.Computer, DbaWorldScreen.Monitoring -> DbaWorldScreen.Computer
            else -> DbaWorldScreen.World
        }
        learningMessage = null
        screen = DbaWorldScreen.LearningCenter
    }

    fun purchaseLearningProduct(
        courseCode: String,
        topicSlug: String,
        productType: LearningProductType,
        russian: Boolean,
    ) {
        val current = world ?: return
        val progress = knowledgeSnapshot.topics[KnowledgeSnapshot.topicKey(courseCode, topicSlug)] ?: return
        val result = LearningEconomyEngine.purchaseProduct(current, progress, productType)
        world = result.world
        trackedWorld = result.world
        learningMessage = if (russian) result.messageRu else result.messageEn
        repository.save(result.world)
    }

    fun purchaseTopicBundle(
        courseCode: String,
        topicSlug: String,
        russian: Boolean,
    ) = purchaseLearningProduct(
        courseCode = courseCode,
        topicSlug = topicSlug,
        productType = LearningProductType.BUNDLE,
        russian = russian,
    )

    fun openLearningHistory() {
        if (world == null) {
            return
        }
        screen = DbaWorldScreen.LearningHistory
    }

    fun openTerminal() {
        terminalReturnScreen = when (screen) {
            DbaWorldScreen.NodeDetails -> DbaWorldScreen.NodeDetails
            DbaWorldScreen.Computer, DbaWorldScreen.Monitoring -> DbaWorldScreen.Computer
            else -> DbaWorldScreen.Workstation
        }
        if (terminalHistory.isEmpty()) {
            terminalHistory += TerminalLine(
                command = null,
                output = "DBA World simulated terminal. Type 'help' to list commands for the active incident.",
            )
        }
        screen = DbaWorldScreen.Terminal
    }

    fun requestHint(russian: Boolean = false) {
        val currentWorld = world ?: return
        val result = IncidentEngine.useHint(currentWorld, russian)
        world = result.world
        trackedWorld = result.world
        latestHint = result.hint
        repository.save(result.world)
    }

    fun executeTerminalCommand(command: String) {
        val currentWorld = world ?: return
        if (command.isBlank()) {
            return
        }

        val activeBefore = currentWorld.incident
        val execution = TerminalEngine.execute(currentWorld, command)
        val resolvedNow = activeBefore?.status == IncidentStatus.ACTIVE &&
            execution.world.incident?.id == activeBefore.id &&
            execution.world.incident?.status == IncidentStatus.RESOLVED
        val rewarded = ProgressionEngine.applyIncidentResolution(
            previous = currentWorld,
            updated = execution.world,
        )
        val advanced = IncidentEngine.afterExecution(
            previous = currentWorld,
            updated = rewarded,
        )

        val output = if (resolvedNow && activeBefore != null) {
            val reward = ProgressionEngine.rewardFor(activeBefore.kind)
            val evaluation = advanced.lastIncidentEvaluation
            buildString {
                append(execution.output)
                append("\n\nRewards: +${reward.playerCredits} Engineer Credits, +${reward.xp} XP, +${reward.companyBalance} company balance.")
                if (evaluation != null) {
                    append("\nScore: ${evaluation.score}/100")
                }
            }
        } else {
            execution.output
        }

        terminalHistory += TerminalLine(command = command.trim(), output = output)
        latestHint = null
        val withWorkday = if (resolvedNow) {
            WorkdayEngine.recordIncidentResolution(advanced)
        } else {
            advanced
        }
        world = withWorkday
        trackedWorld = withWorkday
        repository.save(withWorkday)
    }

    fun onAppResumed() {
        if (activeSinceRealtimeMillis != null) {
            return
        }
        if (trackedWorld == null && repository.hasSavedWorld()) {
            trackedWorld = world ?: repository.load()
        }
        activeSinceRealtimeMillis = SystemClock.elapsedRealtime()
    }

    fun syncActiveTime() {
        val startedAt = activeSinceRealtimeMillis ?: return
        val now = SystemClock.elapsedRealtime()
        val elapsed = (now - startedAt).coerceAtLeast(0L)
        activeSinceRealtimeMillis = now
        if (elapsed == 0L) {
            return
        }

        val current = world ?: trackedWorld ?: return
        val clockAdvanced = WorldClockEngine.advanceByActiveTime(current, elapsed)
        val updated = WorkdayEngine.applyTimeTransitions(current, clockAdvanced)
        trackedWorld = updated
        if (world != null) {
            world = updated
        }
        repository.save(updated)
    }

    fun onAppPaused() {
        syncActiveTime()
        activeSinceRealtimeMillis = null
    }

    fun clearTerminal() {
        terminalHistory.clear()
    }

    fun back(): Boolean = when (screen) {
        DbaWorldScreen.Landing -> false
        DbaWorldScreen.Outdoor -> {
            world?.let { current ->
                trackedWorld = current
                repository.save(current)
            }
            screen = DbaWorldScreen.Landing
            true
        }

        DbaWorldScreen.NodeDetails -> {
            closeNodeDetails()
            true
        }

        DbaWorldScreen.Workstation -> {
            screen = DbaWorldScreen.Outdoor
            true
        }

        DbaWorldScreen.Computer -> {
            screen = DbaWorldScreen.Workstation
            true
        }

        DbaWorldScreen.Monitoring -> {
            screen = DbaWorldScreen.Computer
            true
        }

        DbaWorldScreen.World -> {
            screen = DbaWorldScreen.Workstation
            true
        }

        DbaWorldScreen.Terminal -> {
            screen = terminalReturnScreen
            true
        }

        DbaWorldScreen.LearningCenter -> {
            learningMessage = null
            screen = learningCenterReturnScreen
            true
        }

        DbaWorldScreen.LearningHistory -> {
            screen = DbaWorldScreen.LearningCenter
            true
        }
    }

}
