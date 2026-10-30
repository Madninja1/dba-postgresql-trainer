package dev.dbatrainer.app.game.ui

import androidx.compose.animation.Crossfade
import androidx.compose.animation.core.animateFloat
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.gestures.rememberTransformableState
import androidx.compose.foundation.gestures.transformable
import androidx.compose.foundation.horizontalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.PaddingValues
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Button
import androidx.compose.material3.Card
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Surface
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.OutlinedTextField
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.remember
import androidx.compose.runtime.setValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import dev.dbatrainer.app.UiLanguage
import dev.dbatrainer.app.game.DbaWorldController
import dev.dbatrainer.app.game.DbaWorldScreen
import dev.dbatrainer.app.game.domain.ContentAccessState
import dev.dbatrainer.app.game.domain.IncidentEvaluation
import dev.dbatrainer.app.game.domain.IncidentKind
import dev.dbatrainer.app.game.domain.IncidentSeverity
import dev.dbatrainer.app.game.domain.InfrastructureNode
import dev.dbatrainer.app.game.domain.InfrastructureNodeKind
import dev.dbatrainer.app.game.domain.InfrastructureStatus
import dev.dbatrainer.app.game.domain.IncidentKnowledgeAccess
import dev.dbatrainer.app.game.domain.KnowledgeGateState
import dev.dbatrainer.app.game.domain.KnowledgeMode
import dev.dbatrainer.app.game.domain.KnowledgeRequirementAccess
import dev.dbatrainer.app.game.domain.LearningProductType
import dev.dbatrainer.app.game.domain.LearningTopicOffer
import dev.dbatrainer.app.game.domain.LearningTransaction
import dev.dbatrainer.app.game.domain.WorldClock
import dev.dbatrainer.app.game.domain.WorldRank
import dev.dbatrainer.app.game.domain.WorldState
import dev.dbatrainer.app.game.domain.nextWorldRank
import dev.dbatrainer.app.game.engine.IncidentEngine

@Composable
fun DbaWorldRootScreen(
    controller: DbaWorldController,
    language: UiLanguage,
    onOpenKnowledgeNote: (String, String) -> Unit,
    onOpenKnowledgeQuiz: (String, String) -> Unit,
    onOpenKnowledgeLab: (String, String) -> Unit,
) {
    val russian = language == UiLanguage.Russian
    Crossfade(targetState = controller.screen, label = "dba-world-root") { screen ->
        when (screen) {
            DbaWorldScreen.Landing -> DbaWorldLandingScreen(controller, language)
            DbaWorldScreen.Outdoor -> DbaWorldOutdoorScreen(controller, russian)
            DbaWorldScreen.Workstation -> DbaWorldWorkstationScreen(controller, russian)
            DbaWorldScreen.Computer -> controller.world?.let { world ->
                DbaWorldComputerScreen(
                    world = world,
                    russian = russian,
                    onOpenTerminal = controller::openTerminal,
                    onOpenMonitoring = controller::openMonitoring,
                    onOpenLearningCenter = controller::openLearningCenter,
                    onOpenServerRoom = controller::openServerRoom,
                    onCloseComputer = controller::openWorkstation,
                    onCloseShift = controller::closeShift,
                )
            }
            DbaWorldScreen.Monitoring -> controller.world?.let { world ->
                DbaWorldMonitoringScreen(
                    world = world,
                    russian = russian,
                    onBack = controller::openComputer,
                    onOpenNode = controller::openNodeDetails,
                )
            }
            DbaWorldScreen.World -> DbaWorldOverviewScreen(
                controller = controller,
                language = language,
            )
            DbaWorldScreen.NodeDetails -> DbaWorldNodeDetailsScreen(
                controller = controller,
                language = language,
                onOpenKnowledgeNote = onOpenKnowledgeNote,
                onOpenKnowledgeQuiz = onOpenKnowledgeQuiz,
            )
            DbaWorldScreen.Terminal -> DbaWorldTerminalScreen(controller, language)
            DbaWorldScreen.LearningCenter -> DbaWorldLearningCenterScreen(
                controller = controller,
                language = language,
                onOpenKnowledgeNote = onOpenKnowledgeNote,
                onOpenKnowledgeQuiz = onOpenKnowledgeQuiz,
                onOpenKnowledgeLab = onOpenKnowledgeLab,
            )
            DbaWorldScreen.LearningHistory -> DbaWorldLearningHistoryScreen(
                controller = controller,
                language = language,
            )
        }
    }
}

@Composable
private fun DbaWorldLandingScreen(
    controller: DbaWorldController,
    language: UiLanguage,
) {
    val russian = language == UiLanguage.Russian

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(24.dp),
        verticalArrangement = Arrangement.spacedBy(16.dp),
    ) {
        Text(
            text = "DBA World",
            style = MaterialTheme.typography.headlineLarge,
            fontWeight = FontWeight.Bold,
        )
        Text(
            text = if (russian) {
                "Инженерный симулятор инфраструктуры"
            } else {
                "Infrastructure engineering simulator"
            },
            style = MaterialTheme.typography.bodyLarge,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )

        if (controller.canContinue) {
            Button(
                onClick = controller::continueGame,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(if (russian) "Продолжить" else "Continue")
            }
        }

        OutlinedButton(
            onClick = controller::newGame,
            modifier = Modifier.fillMaxWidth(),
        ) {
            Text(if (russian) "Новая игра" else "New Game")
        }
    }
}

@Composable
private fun DbaWorldOverviewScreen(
    controller: DbaWorldController,
    language: UiLanguage,
) {
    val russian = language == UiLanguage.Russian
    val world = controller.world ?: return

    Column(modifier = Modifier.fillMaxSize()) {
        WorldHud(world = world, russian = russian)

        ServerRoomScene(
            world = world,
            russian = russian,
            onNodeClick = controller::openNodeDetails,
            modifier = Modifier
                .fillMaxWidth()
                .weight(1f),
        )

        Surface(tonalElevation = 6.dp) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 12.dp, vertical = 10.dp),
                horizontalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                OutlinedButton(
                    onClick = controller::openWorkstation,
                    modifier = Modifier.weight(1f),
                ) {
                    Text(if (russian) "← Рабочее место" else "← Workstation")
                }
                OutlinedButton(
                    onClick = controller::openLearningCenter,
                    modifier = Modifier.weight(1f),
                ) {
                    Text(if (russian) "Учебный центр" else "Learning Center")
                }
                world.lastIncidentEvaluation?.let { evaluation ->
                    Surface(
                        shape = RoundedCornerShape(12.dp),
                        tonalElevation = 2.dp,
                        modifier = Modifier.weight(1f),
                    ) {
                        Column(modifier = Modifier.padding(horizontal = 12.dp, vertical = 8.dp)) {
                            Text(
                                text = if (russian) "Последний инцидент" else "Last incident",
                                style = MaterialTheme.typography.labelSmall,
                                color = MaterialTheme.colorScheme.onSurfaceVariant,
                            )
                            Text(
                                text = "${evaluation.score}/100 · ${evaluation.diagnosis.name}",
                                style = MaterialTheme.typography.labelLarge,
                                fontWeight = FontWeight.SemiBold,
                            )
                        }
                    }
                }
            }
        }
    }
}

@Composable
private fun WorldHud(
    world: WorldState,
    russian: Boolean,
) {
    Surface(tonalElevation = 8.dp) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 14.dp, vertical = 10.dp),
            verticalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Column {
                    Text(
                        text = world.company.name,
                        style = MaterialTheme.typography.titleLarge,
                        fontWeight = FontWeight.Bold,
                    )
                    Text(
                        text = clockLabel(world.clock, russian),
                        style = MaterialTheme.typography.bodySmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
                Column(horizontalAlignment = Alignment.End) {
                    Text(
                        text = "€${world.company.balance}",
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.SemiBold,
                    )
                    Text(
                        text = "${world.player.credits} CR · ${rankLabel(world.player.worldRank, russian)}",
                        style = MaterialTheme.typography.labelSmall,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                    )
                }
            }
        }
    }
}

@Composable
private fun ServerRoomScene(
    world: WorldState,
    russian: Boolean,
    onNodeClick: (String) -> Unit,
    modifier: Modifier = Modifier,
) {
    var scale by remember { mutableStateOf(1f) }
    var sceneOffset by remember { mutableStateOf(Offset.Zero) }
    val transformState = rememberTransformableState { zoomChange, panChange, _ ->
        scale = (scale * zoomChange).coerceIn(0.85f, 1.65f)
        sceneOffset = sceneOffset + panChange
    }
    val colors = MaterialTheme.colorScheme
    val application = world.nodes.firstOrNull { it.kind == InfrastructureNodeKind.APPLICATION }
    val database = world.nodes.firstOrNull { it.kind == InfrastructureNodeKind.POSTGRESQL }

    Box(
        modifier = modifier
            .background(colors.surfaceVariant.copy(alpha = 0.45f)),
    ) {
        Canvas(modifier = Modifier.matchParentSize()) {
            val wallHeight = size.height * 0.28f
            drawRect(colors.surface, size = androidx.compose.ui.geometry.Size(size.width, wallHeight))
            drawRect(
                colors.surfaceVariant.copy(alpha = 0.55f),
                topLeft = Offset(0f, wallHeight),
                size = androidx.compose.ui.geometry.Size(size.width, size.height - wallHeight),
            )

            val vanishing = Offset(size.width / 2f, wallHeight)
            val spacing = size.width / 8f
            for (i in -3..3) {
                val x = size.width / 2f + i * spacing
                drawLine(
                    color = colors.outlineVariant.copy(alpha = 0.45f),
                    start = vanishing,
                    end = Offset(x, size.height),
                    strokeWidth = 1.5f,
                )
            }
            var y = wallHeight + 35f
            var gap = 44f
            while (y < size.height) {
                drawLine(
                    color = colors.outlineVariant.copy(alpha = 0.38f),
                    start = Offset(0f, y),
                    end = Offset(size.width, y),
                    strokeWidth = 1f,
                )
                y += gap
                gap *= 1.14f
            }
        }

        Box(
            modifier = Modifier
                .fillMaxSize()
                .graphicsLayer(
                    scaleX = scale,
                    scaleY = scale,
                    translationX = sceneOffset.x,
                    translationY = sceneOffset.y,
                )
                .transformable(transformState),
        ) {
            RackCabinet(
                title = "RACK A",
                modifier = Modifier
                    .align(Alignment.Center)
                    .offset(x = (-58).dp, y = 6.dp),
            ) {
                if (application != null) {
                    ServerUnit(
                        node = application,
                        russian = russian,
                        hasLocalAlert = application.status != InfrastructureStatus.HEALTHY,
                        onClick = { onNodeClick(application.id) },
                    )
                }
                if (database != null) {
                    ServerUnit(
                        node = database,
                        russian = russian,
                        hasLocalAlert = world.incident != null || database.status != InfrastructureStatus.HEALTHY,
                        onClick = { onNodeClick(database.id) },
                    )
                }
                EmptyRackSlot(label = if (russian) "свободно" else "free")
                EmptyRackSlot(label = if (russian) "свободно" else "free")
            }

            FutureRack(
                russian = russian,
                modifier = Modifier
                    .align(Alignment.Center)
                    .offset(x = 118.dp, y = (-18).dp),
            )

            FloorDevice(
                title = if (russian) "СЕТЬ" else "NETWORK",
                subtitle = "SW-01",
                modifier = Modifier
                    .align(Alignment.BottomCenter)
                    .offset(x = 92.dp, y = (-44).dp),
            )

            FloorDevice(
                title = "UPS",
                subtitle = if (russian) "питание" else "power",
                modifier = Modifier
                    .align(Alignment.BottomStart)
                    .offset(x = 20.dp, y = (-36).dp),
            )
        }

        Surface(
            shape = RoundedCornerShape(12.dp),
            color = colors.surface.copy(alpha = 0.92f),
            tonalElevation = 3.dp,
            modifier = Modifier
                .align(Alignment.TopStart)
                .padding(10.dp),
        ) {
            Text(
                text = if (russian) {
                    "Серверная · перетаскивайте и масштабируйте"
                } else {
                    "Server room · drag and pinch to zoom"
                },
                modifier = Modifier.padding(horizontal = 10.dp, vertical = 6.dp),
                style = MaterialTheme.typography.labelSmall,
            )
        }

        OutlinedButton(
            onClick = {
                scale = 1f
                sceneOffset = Offset.Zero
            },
            modifier = Modifier
                .align(Alignment.TopEnd)
                .padding(8.dp),
        ) {
            Text(if (russian) "Камера" else "Camera")
        }
    }
}

@Composable
private fun RackCabinet(
    title: String,
    modifier: Modifier = Modifier,
    content: @Composable () -> Unit,
) {
    val colors = MaterialTheme.colorScheme
    Surface(
        modifier = modifier
            .width(178.dp)
            .height(300.dp)
            .graphicsLayer(rotationZ = -1.2f),
        shape = RoundedCornerShape(8.dp),
        color = colors.surface,
        shadowElevation = 12.dp,
        tonalElevation = 4.dp,
    ) {
        Column(
            modifier = Modifier
                .border(2.dp, colors.outline, RoundedCornerShape(8.dp))
                .padding(8.dp),
            verticalArrangement = Arrangement.spacedBy(7.dp),
        ) {
            Text(
                text = title,
                style = MaterialTheme.typography.labelLarge,
                fontWeight = FontWeight.Bold,
            )
            Spacer(
                modifier = Modifier
                    .fillMaxWidth()
                    .height(2.dp)
                    .background(colors.outlineVariant),
            )
            content()
        }
    }
}

@Composable
private fun ServerUnit(
    node: InfrastructureNode,
    russian: Boolean,
    hasLocalAlert: Boolean,
    onClick: () -> Unit,
) {
    val colors = MaterialTheme.colorScheme
    val statusColor = when (node.status) {
        InfrastructureStatus.HEALTHY -> colors.primary
        InfrastructureStatus.WARNING -> colors.tertiary
        InfrastructureStatus.DOWN -> colors.error
    }

    Box {
        Surface(
            modifier = Modifier
                .fillMaxWidth()
                .height(53.dp)
                .clickable(onClick = onClick),
            shape = RoundedCornerShape(5.dp),
            color = colors.surfaceVariant,
            tonalElevation = 2.dp,
        ) {
            Row(
                modifier = Modifier
                    .border(1.dp, statusColor.copy(alpha = 0.75f), RoundedCornerShape(5.dp))
                    .padding(horizontal = 9.dp, vertical = 7.dp),
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                Canvas(modifier = Modifier.size(11.dp)) {
                    drawCircle(statusColor)
                    drawCircle(colors.onSurface.copy(alpha = 0.18f), radius = size.minDimension * 0.24f)
                }
                Column(modifier = Modifier.weight(1f)) {
                    Text(
                        text = node.id,
                        style = MaterialTheme.typography.labelLarge,
                        fontWeight = FontWeight.Bold,
                    )
                    Row(horizontalArrangement = Arrangement.spacedBy(4.dp)) {
                        repeat(3) { ledIndex ->
                            Canvas(modifier = Modifier.size(5.dp)) {
                                val ledColor = if (ledIndex == 0) statusColor else colors.primary.copy(alpha = 0.7f)
                                drawCircle(ledColor)
                            }
                        }
                    }
                    Text(
                        text = statusLabel(node.status, russian),
                        style = MaterialTheme.typography.labelSmall,
                        color = colors.onSurfaceVariant,
                    )
                }
                Column(horizontalAlignment = Alignment.End) {
                    Text(
                        text = statusLabel(node.status, russian),
                        style = MaterialTheme.typography.labelSmall,
                        color = statusColor,
                        fontWeight = FontWeight.SemiBold,
                    )
                    Text(
                        text = "CPU ${node.metrics.cpuPercent}%",
                        style = MaterialTheme.typography.labelSmall,
                        fontFamily = FontFamily.Monospace,
                        color = colors.onSurfaceVariant,
                    )
                }
            }
        }

        if (hasLocalAlert) {
            LocalAlertBadge(
                critical = node.status == InfrastructureStatus.DOWN,
                modifier = Modifier
                    .align(Alignment.TopEnd)
                    .offset(x = 8.dp, y = (-9).dp),
            )
        }
    }
}

@Composable
private fun LocalAlertBadge(
    critical: Boolean,
    modifier: Modifier = Modifier,
) {
    val color = if (critical) MaterialTheme.colorScheme.error else MaterialTheme.colorScheme.tertiary
    val contentColor = if (critical) MaterialTheme.colorScheme.onError else MaterialTheme.colorScheme.onTertiary
    val transition = androidx.compose.animation.core.rememberInfiniteTransition(label = "local-alert")
    val pulse by transition.animateFloat(
        initialValue = 1f,
        targetValue = 1.12f,
        animationSpec = androidx.compose.animation.core.infiniteRepeatable(
            animation = androidx.compose.animation.core.tween(650),
            repeatMode = androidx.compose.animation.core.RepeatMode.Reverse,
        ),
        label = "local-alert-pulse",
    )
    Box(
        modifier = modifier
            .graphicsLayer(scaleX = pulse, scaleY = pulse)
            .size(27.dp)
            .background(color, CircleShape)
            .border(2.dp, MaterialTheme.colorScheme.surface, CircleShape),
        contentAlignment = Alignment.Center,
    ) {
        Text(
            text = "!",
            color = contentColor,
            fontWeight = FontWeight.Black,
        )
    }
}

@Composable
private fun EmptyRackSlot(
    label: String,
) {
    Box(
        modifier = Modifier
            .fillMaxWidth()
            .height(45.dp)
            .border(
                1.dp,
                MaterialTheme.colorScheme.outlineVariant,
                RoundedCornerShape(5.dp),
            ),
        contentAlignment = Alignment.Center,
    ) {
        Text(
            text = label,
            style = MaterialTheme.typography.labelSmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant.copy(alpha = 0.55f),
        )
    }
}

@Composable
private fun FutureRack(
    russian: Boolean,
    modifier: Modifier = Modifier,
) {
    Surface(
        modifier = modifier
            .width(126.dp)
            .height(235.dp)
            .graphicsLayer(rotationZ = 1.8f),
        shape = RoundedCornerShape(8.dp),
        color = MaterialTheme.colorScheme.surface.copy(alpha = 0.72f),
        shadowElevation = 5.dp,
    ) {
        Column(
            modifier = Modifier
                .border(1.dp, MaterialTheme.colorScheme.outlineVariant, RoundedCornerShape(8.dp))
                .padding(10.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Text("RACK B", fontWeight = FontWeight.Bold, style = MaterialTheme.typography.labelLarge)
            Text(
                text = if (russian) "Расширение инфраструктуры" else "Infrastructure expansion",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            repeat(3) {
                EmptyRackSlot(label = if (russian) "закрыто" else "locked")
            }
        }
    }
}

@Composable
private fun FloorDevice(
    title: String,
    subtitle: String,
    modifier: Modifier = Modifier,
) {
    Surface(
        modifier = modifier.width(92.dp),
        shape = RoundedCornerShape(6.dp),
        tonalElevation = 5.dp,
        shadowElevation = 5.dp,
    ) {
        Column(
            modifier = Modifier.padding(horizontal = 10.dp, vertical = 8.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
        ) {
            Text(title, style = MaterialTheme.typography.labelMedium, fontWeight = FontWeight.Bold)
            Text(
                subtitle,
                style = MaterialTheme.typography.labelSmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

@Composable
private fun DbaWorldNodeDetailsScreen(
    controller: DbaWorldController,
    language: UiLanguage,
    onOpenKnowledgeNote: (String, String) -> Unit,
    onOpenKnowledgeQuiz: (String, String) -> Unit,
) {
    val russian = language == UiLanguage.Russian
    val world = controller.world ?: return
    val node = controller.selectedNode ?: return
    var postgresqlExpanded by remember(node.id) {
        mutableStateOf(node.kind == InfrastructureNodeKind.POSTGRESQL)
    }

    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        item {
            OutlinedButton(onClick = controller::closeNodeDetails) {
                Text(if (russian) "← Серверная" else "← Server room")
            }
        }

        item {
            ServerInspectionHeader(node = node, russian = russian)
        }

        item {
            Card(modifier = Modifier.fillMaxWidth()) {
                Column(
                    modifier = Modifier.padding(16.dp),
                    verticalArrangement = Arrangement.spacedBy(12.dp),
                ) {
                    Text(
                        text = if (russian) "Ресурсы сервера" else "Server resources",
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.SemiBold,
                    )
                    MetricRow("CPU", node.metrics.cpuPercent)
                    MetricRow("RAM", node.metrics.ramPercent)
                    MetricRow(if (russian) "Диск" else "Disk", node.metrics.diskPercent)
                }
            }
        }

        item {
            ServicesPanel(
                node = node,
                russian = russian,
                postgresqlExpanded = postgresqlExpanded,
                onPostgresqlClick = { postgresqlExpanded = !postgresqlExpanded },
            )
        }

        if (node.kind == InfrastructureNodeKind.POSTGRESQL && postgresqlExpanded) {
            item {
                PostgresqlComponentPanel(
                    controller = controller,
                    world = world,
                    russian = russian,
                )
            }

        }

        if (node.kind == InfrastructureNodeKind.APPLICATION && node.status != InfrastructureStatus.HEALTHY) {
            item {
                Card(modifier = Modifier.fillMaxWidth()) {
                    Column(
                        modifier = Modifier.padding(16.dp),
                        verticalArrangement = Arrangement.spacedBy(6.dp),
                    ) {
                        Text(
                            text = if (russian) "Зависимость нарушена" else "Dependency degraded",
                            fontWeight = FontWeight.SemiBold,
                        )
                        Text(
                            text = if (russian) {
                                "Приложение сообщает об ошибках подключения. Проверьте сервер базы данных db-primary."
                            } else {
                                "The application reports connection errors. Inspect the db-primary database server."
                            },
                        )
                    }
                }
            }
        }
    }
}

@Composable
private fun ServerInspectionHeader(
    node: InfrastructureNode,
    russian: Boolean,
) {
    val statusColor = when (node.status) {
        InfrastructureStatus.HEALTHY -> MaterialTheme.colorScheme.primary
        InfrastructureStatus.WARNING -> MaterialTheme.colorScheme.tertiary
        InfrastructureStatus.DOWN -> MaterialTheme.colorScheme.error
    }
    Surface(
        modifier = Modifier.fillMaxWidth(),
        shape = RoundedCornerShape(14.dp),
        tonalElevation = 5.dp,
    ) {
        Row(
            modifier = Modifier.padding(16.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Canvas(modifier = Modifier.size(20.dp)) {
                drawCircle(statusColor)
            }
            Column(modifier = Modifier.weight(1f)) {
                Text(node.id, style = MaterialTheme.typography.headlineSmall, fontWeight = FontWeight.Bold)
                Text(
                    text = if (node.kind == InfrastructureNodeKind.POSTGRESQL) {
                        if (russian) "Linux-сервер · база данных" else "Linux server · database"
                    } else {
                        if (russian) "Linux-сервер · приложение" else "Linux server · application"
                    },
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            Text(
                text = statusLabel(node.status, russian),
                color = statusColor,
                fontWeight = FontWeight.Bold,
            )
        }
    }
}

@Composable
private fun ServicesPanel(
    node: InfrastructureNode,
    russian: Boolean,
    postgresqlExpanded: Boolean,
    onPostgresqlClick: () -> Unit,
) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Text(
                text = if (russian) "Сервисы" else "Services",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )
            if (node.kind == InfrastructureNodeKind.APPLICATION) {
                ServiceStatusRow("application.service", node.status, false, null)
                ServiceStatusRow("ssh.service", InfrastructureStatus.HEALTHY, false, null)
            } else {
                ServiceStatusRow(
                    name = "postgresql.service",
                    status = node.status,
                    hasAlert = node.status != InfrastructureStatus.HEALTHY,
                    onClick = onPostgresqlClick,
                    expanded = postgresqlExpanded,
                )
                ServiceStatusRow("ssh.service", InfrastructureStatus.HEALTHY, false, null)
                ServiceStatusRow("node_exporter.service", InfrastructureStatus.HEALTHY, false, null)
            }
        }
    }
}

@Composable
private fun ServiceStatusRow(
    name: String,
    status: InfrastructureStatus,
    hasAlert: Boolean,
    onClick: (() -> Unit)?,
    expanded: Boolean = false,
) {
    val statusColor = when (status) {
        InfrastructureStatus.HEALTHY -> MaterialTheme.colorScheme.primary
        InfrastructureStatus.WARNING -> MaterialTheme.colorScheme.tertiary
        InfrastructureStatus.DOWN -> MaterialTheme.colorScheme.error
    }
    val modifier = if (onClick != null) {
        Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick)
    } else {
        Modifier.fillMaxWidth()
    }
    Row(
        modifier = modifier.padding(vertical = 7.dp),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(10.dp),
    ) {
        Canvas(modifier = Modifier.size(10.dp)) { drawCircle(statusColor) }
        Text(name, modifier = Modifier.weight(1f), fontFamily = FontFamily.Monospace)
        if (hasAlert) {
            LocalAlertBadge(critical = status == InfrastructureStatus.DOWN)
        }
        if (onClick != null) {
            Text(if (expanded) "▲" else "▼", style = MaterialTheme.typography.labelSmall)
        }
    }
}

@Composable
private fun PostgresqlComponentPanel(
    controller: DbaWorldController,
    world: WorldState,
    russian: Boolean,
) {
    val incident = world.incident
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(10.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    text = "PostgreSQL",
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Bold,
                )
                if (incident != null) {
                    LocalAlertBadge(critical = incident.severity == IncidentSeverity.CRITICAL)
                }
            }
            ValueRow("shared_buffers", world.postgresqlConfig.sharedBuffers)

            if (incident != null) {
                Text(
                    text = incidentTitle(incident.kind, russian),
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                )
                Text(
                    text = if (russian) IncidentEngine.summaryRu(incident.kind) else IncidentEngine.summary(incident.kind),
                )
                Text(
                    text = "${severityLabel(incident.severity, russian)} · " + if (russian) {
                        "действий: ${incident.actionCount} · подсказок: ${incident.hintsUsed}"
                    } else {
                        "actions: ${incident.actionCount} · hints: ${incident.hintsUsed}"
                    },
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                controller.latestHint?.let { hint ->
                    Text(
                        text = (if (russian) "Подсказка: " else "Hint: ") + hint,
                        color = MaterialTheme.colorScheme.primary,
                    )
                }
                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                ) {
                    Button(
                        onClick = controller::openTerminal,
                        modifier = Modifier.weight(1f),
                    ) {
                        Text(if (russian) "Терминал" else "Terminal")
                    }
                    OutlinedButton(
                        onClick = { controller.requestHint(russian) },
                        modifier = Modifier.weight(1f),
                    ) {
                        Text(if (russian) "Подсказка" else "Hint")
                    }
                }
            } else {
                Text(
                    text = if (russian) {
                        "PostgreSQL работает штатно. Активных событий для сервиса нет."
                    } else {
                        "PostgreSQL is operating normally. There are no active service events."
                    },
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                OutlinedButton(
                    onClick = controller::openTerminal,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text(if (russian) "Открыть терминал" else "Open terminal")
                }
            }
        }
    }
}

@Composable
private fun DbaWorldLearningCenterScreen(
    controller: DbaWorldController,
    language: UiLanguage,
    onOpenKnowledgeNote: (String, String) -> Unit,
    onOpenKnowledgeQuiz: (String, String) -> Unit,
    onOpenKnowledgeLab: (String, String) -> Unit,
) {
    val russian = language == UiLanguage.Russian
    val world = controller.world ?: return
    val offers = controller.learningOffers
    val courses = offers.groupBy { it.courseCode }

    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        item {
            Text(
                text = if (russian) "Учебный центр" else "Learning Center",
                style = MaterialTheme.typography.headlineMedium,
                fontWeight = FontWeight.Bold,
            )
            Text(
                text = if (russian) {
                    "Engineer Credits: ${world.player.credits} CR · материалы можно покупать отдельно или полным блоком со скидкой."
                } else {
                    "Engineer Credits: ${world.player.credits} CR · buy materials separately or unlock a discounted full bundle."
                },
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                OutlinedButton(
                    onClick = { controller.back() },
                    modifier = Modifier.weight(1f),
                ) {
                    Text(if (russian) "Назад" else "Back")
                }
                OutlinedButton(
                    onClick = controller::openLearningHistory,
                    modifier = Modifier.weight(1f),
                ) {
                    Text(if (russian) "История операций" else "Transactions")
                }
            }
        }

        controller.learningMessage?.let { message ->
            item {
                Card(modifier = Modifier.fillMaxWidth()) {
                    Text(
                        text = message,
                        modifier = Modifier.padding(12.dp),
                        color = MaterialTheme.colorScheme.primary,
                    )
                }
            }
        }

        courses.forEach { (courseCode, courseOffers) ->
            item(key = "course-$courseCode") {
                Text(
                    text = learningCourseLabel(courseCode),
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.primary,
                )
            }
            items(
                items = courseOffers,
                key = { offer -> "${offer.courseCode}:${offer.topicSlug}" },
            ) { offer ->
                LearningTopicOfferCard(
                    offer = offer,
                    credits = world.player.credits,
                    russian = russian,
                    onPurchase = { productType ->
                        controller.purchaseLearningProduct(
                            courseCode = offer.courseCode,
                            topicSlug = offer.topicSlug,
                            productType = productType,
                            russian = russian,
                        )
                    },
                    onOpenNote = { onOpenKnowledgeNote(offer.courseCode, offer.topicSlug) },
                    onOpenQuiz = { onOpenKnowledgeQuiz(offer.courseCode, offer.topicSlug) },
                    onOpenLab = { onOpenKnowledgeLab(offer.courseCode, offer.topicSlug) },
                )
            }
        }
    }
}

@Composable
private fun LearningTopicOfferCard(
    offer: LearningTopicOffer,
    credits: Int,
    russian: Boolean,
    onPurchase: (LearningProductType) -> Unit,
    onOpenNote: () -> Unit,
    onOpenQuiz: () -> Unit,
    onOpenLab: () -> Unit,
) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
            ) {
                Text(
                    text = "${offer.topicNumber}. ${offer.title}",
                    modifier = Modifier.weight(1f),
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                )
                Text(
                    text = contentAccessLabel(offer.accessState, russian),
                    color = MaterialTheme.colorScheme.primary,
                    fontWeight = FontWeight.SemiBold,
                )
            }

            LearningProductRow(
                title = if (russian) "Конспект" else "Notes",
                state = offer.notesAccess,
                price = offer.prices.notes,
                credits = credits,
                russian = russian,
                onPurchase = { onPurchase(LearningProductType.NOTES) },
                onOpen = onOpenNote,
            )
            LearningProductRow(
                title = if (russian) "Тест" else "Test",
                state = offer.testAccess,
                price = offer.prices.test,
                credits = credits,
                russian = russian,
                onPurchase = { onPurchase(LearningProductType.TEST) },
                onOpen = onOpenQuiz,
            )
            offer.labAccess?.let { labAccess ->
                LearningProductRow(
                    title = if (russian) "Лабораторная" else "Lab",
                    state = labAccess,
                    price = offer.prices.lab,
                    credits = credits,
                    russian = russian,
                    onPurchase = { onPurchase(LearningProductType.LAB) },
                    onOpen = onOpenLab,
                )
            }

            if (
                offer.accessState != ContentAccessState.FREE &&
                offer.accessState != ContentAccessState.MASTERED &&
                !offer.bundlePurchased &&
                !offer.allAvailableProductsAccessible
            ) {
                val baseBundlePrice = offer.prices.bundle
                val upgradePrice = offer.bundleUpgradePrice
                Text(
                    text = if (russian) {
                        "Полный блок: $baseBundlePrice CR · скидка ${offer.prices.bundleDiscount} CR относительно отдельных покупок."
                    } else {
                        "Full bundle: $baseBundlePrice CR · save ${offer.prices.bundleDiscount} CR versus separate purchases."
                    },
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
                Button(
                    onClick = { onPurchase(LearningProductType.BUNDLE) },
                    enabled = credits >= upgradePrice,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text(
                        when {
                            upgradePrice == 0 -> if (russian) {
                                "Завершить полный блок"
                            } else {
                                "Complete full bundle"
                            }
                            upgradePrice < baseBundlePrice -> if (russian) {
                                "Доплатить за полный блок · $upgradePrice CR"
                            } else {
                                "Upgrade to full bundle · $upgradePrice CR"
                            }
                            credits >= upgradePrice -> if (russian) {
                                "Открыть полный блок · $upgradePrice CR"
                            } else {
                                "Unlock full bundle · $upgradePrice CR"
                            }
                            else -> if (russian) {
                                "Нужно $upgradePrice CR"
                            } else {
                                "Requires $upgradePrice CR"
                            }
                        },
                    )
                }
            }
        }
    }
}

@Composable
private fun LearningProductRow(
    title: String,
    state: ContentAccessState,
    price: Int,
    credits: Int,
    russian: Boolean,
    onPurchase: () -> Unit,
    onOpen: () -> Unit,
) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.spacedBy(8.dp),
    ) {
        Column(modifier = Modifier.weight(1f)) {
            Text(title, fontWeight = FontWeight.SemiBold)
            Text(
                text = contentAccessLabel(state, russian),
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
        if (state == ContentAccessState.LOCKED) {
            Button(
                onClick = onPurchase,
                enabled = credits >= price,
            ) {
                Text(
                    if (credits >= price) {
                        if (russian) "Купить · $price CR" else "Buy · $price CR"
                    } else {
                        if (russian) "Нужно $price CR" else "Need $price CR"
                    },
                )
            }
        } else {
            OutlinedButton(onClick = onOpen) {
                Text(if (russian) "Открыть" else "Open")
            }
        }
    }
}

@Composable
private fun DbaWorldLearningHistoryScreen(
    controller: DbaWorldController,
    language: UiLanguage,
) {
    val russian = language == UiLanguage.Russian
    val world = controller.world ?: return
    val offersByKey = controller.learningOffers.associateBy { offer ->
        "${offer.courseCode}::${offer.topicSlug}"
    }

    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(10.dp),
    ) {
        item {
            Text(
                text = if (russian) "История операций" else "Transaction History",
                style = MaterialTheme.typography.headlineMedium,
                fontWeight = FontWeight.Bold,
            )
            Text(
                text = if (russian) {
                    "Текущий баланс: ${world.player.credits} CR"
                } else {
                    "Current balance: ${world.player.credits} CR"
                },
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
            OutlinedButton(onClick = { controller.back() }) {
                Text(if (russian) "Назад в Учебный центр" else "Back to Learning Center")
            }
        }

        if (world.learningTransactions.isEmpty()) {
            item {
                Card(modifier = Modifier.fillMaxWidth()) {
                    Text(
                        text = if (russian) {
                            "Покупок за Engineer Credits пока не было."
                        } else {
                            "No Engineer Credits purchases yet."
                        },
                        modifier = Modifier.padding(16.dp),
                    )
                }
            }
        } else {
            items(
                items = world.learningTransactions.asReversed(),
                key = { transaction -> transaction.id },
            ) { transaction ->
                LearningTransactionCard(
                    transaction = transaction,
                    title = offersByKey[transaction.topicKey]?.title ?: transaction.topicKey,
                    russian = russian,
                )
            }
        }
    }
}

@Composable
private fun LearningTransactionCard(
    transaction: LearningTransaction,
    title: String,
    russian: Boolean,
) {
    val totalMinutes = transaction.worldTotalMinutes.coerceAtLeast(0)
    val clock = WorldClock(
        day = totalMinutes / (24 * 60) + 1,
        minuteOfDay = totalMinutes % (24 * 60),
    )
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(12.dp),
            verticalArrangement = Arrangement.spacedBy(4.dp),
        ) {
            Text(
                text = title,
                fontWeight = FontWeight.SemiBold,
            )
            Text(
                text = "${learningProductLabel(transaction.productType, russian)} · -${transaction.price} CR",
                color = MaterialTheme.colorScheme.primary,
            )
            Text(
                text = "${clockLabel(clock, russian)} · ${if (russian) "баланс" else "balance"}: ${transaction.balanceAfter} CR",
                style = MaterialTheme.typography.bodySmall,
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )
        }
    }
}

private fun learningProductLabel(
    type: LearningProductType,
    russian: Boolean,
): String = when (type) {
    LearningProductType.NOTES -> if (russian) "Конспект" else "Notes"
    LearningProductType.TEST -> if (russian) "Тест" else "Test"
    LearningProductType.LAB -> if (russian) "Лабораторная" else "Lab"
    LearningProductType.BUNDLE -> if (russian) "Полный блок" else "Full bundle"
}

private fun contentAccessLabel(
    state: ContentAccessState,
    russian: Boolean,
): String = when (state) {
    ContentAccessState.FREE -> if (russian) "Бесплатно" else "Free"
    ContentAccessState.LOCKED -> if (russian) "Закрыто" else "Locked"
    ContentAccessState.PURCHASED -> if (russian) "Куплено" else "Purchased"
    ContentAccessState.MASTERED -> if (russian) "Освоено" else "Mastered"
}

private fun learningCourseLabel(courseCode: String): String = when (courseCode) {
    "linux-dba-1" -> "Linux DBA"
    "dba-1" -> "DBA-1"
    "sql-1" -> "SQL"
    "psql-1" -> "psql"
    "plpgsql-1" -> "PL/pgSQL"
    else -> courseCode.uppercase()
}

@Composable
private fun DbaWorldTerminalScreen(
    controller: DbaWorldController,
    language: UiLanguage,
) {
    val russian = language == UiLanguage.Russian
    val world = controller.world ?: return
    val knowledgeMode = controller.activeKnowledgeAccess?.mode ?: KnowledgeMode.GUIDED
    var command by remember { mutableStateOf("") }

    Column(
        modifier = Modifier
            .fillMaxSize()
            .padding(16.dp),
        verticalArrangement = Arrangement.spacedBy(10.dp),
    ) {
        Text(
            text = if (russian) "Терминал" else "Terminal",
            style = MaterialTheme.typography.headlineMedium,
            fontWeight = FontWeight.Bold,
        )
        Text(
            text = "${clockLabel(world.clock, russian)} · ${world.timeZoneId}",
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Text(
            text = if (russian) {
                if (knowledgeMode == KnowledgeMode.GUIDED) {
                    "Guided Mode · команды-подсказки доступны. Игровое время идёт, пока приложение активно."
                } else {
                    "Normal Mode · команды-подсказки скрыты. Используйте свои знания или help."
                }
            } else {
                if (knowledgeMode == KnowledgeMode.GUIDED) {
                    "Guided Mode · command suggestions are available. Game time advances while the app is active."
                } else {
                    "Normal Mode · command suggestions are hidden. Use your knowledge or help."
                }
            },
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )

        controller.latestHint?.let { hint ->
            Card(modifier = Modifier.fillMaxWidth()) {
                Text(
                    text = (if (russian) "Подсказка: " else "Hint: ") + hint,
                    modifier = Modifier.padding(12.dp),
                )
            }
        }

        LazyColumn(
            modifier = Modifier
                .weight(1f)
                .fillMaxWidth(),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            items(controller.terminalHistory) { line ->
                Column(verticalArrangement = Arrangement.spacedBy(2.dp)) {
                    line.command?.let {
                        Text(
                            text = "dba@db-primary:$ $it",
                            fontFamily = FontFamily.Monospace,
                            fontWeight = FontWeight.SemiBold,
                        )
                    }
                    if (line.output.isNotBlank()) {
                        Text(
                            text = line.output,
                            fontFamily = FontFamily.Monospace,
                            style = MaterialTheme.typography.bodySmall,
                        )
                    }
                }
            }
        }

        if (controller.terminalSuggestions.isNotEmpty()) {
            Row(
                modifier = Modifier
                    .fillMaxWidth()
                    .horizontalScroll(rememberScrollState()),
                horizontalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                controller.terminalSuggestions.forEach { suggestion ->
                    OutlinedButton(onClick = { command = suggestion }) {
                        Text(suggestion, fontFamily = FontFamily.Monospace)
                    }
                }
            }
        }

        OutlinedTextField(
            value = command,
            onValueChange = { command = it },
            modifier = Modifier.fillMaxWidth(),
            singleLine = true,
            label = { Text(if (russian) "Команда" else "Command") },
            textStyle = MaterialTheme.typography.bodyMedium.copy(fontFamily = FontFamily.Monospace),
        )

        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Button(
                onClick = {
                    controller.executeTerminalCommand(command)
                    command = ""
                },
                enabled = command.isNotBlank(),
                modifier = Modifier.weight(1f),
            ) {
                Text(if (russian) "Выполнить" else "Run")
            }
            OutlinedButton(
                onClick = { controller.requestHint(russian) },
                modifier = Modifier.weight(1f),
            ) {
                Text(if (russian) "Подсказка" else "Hint")
            }
            OutlinedButton(
                onClick = controller::clearTerminal,
            ) {
                Text(if (russian) "Очистить" else "Clear")
            }
        }
    }
}

@Composable
private fun KnowledgeGateCard(
    access: IncidentKnowledgeAccess,
    russian: Boolean,
    guidedTip: String?,
    onOpenNote: (String, String) -> Unit,
    onOpenQuiz: (String, String) -> Unit,
) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(10.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
            ) {
                Text(
                    text = "Knowledge Gate",
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.SemiBold,
                )
                Text(
                    text = if (access.mode == KnowledgeMode.NORMAL) "Normal Mode" else "Guided Mode",
                    color = MaterialTheme.colorScheme.primary,
                    fontWeight = FontWeight.SemiBold,
                )
            }

            Text(
                text = gateDescription(access.gateState, russian),
                color = MaterialTheme.colorScheme.onSurfaceVariant,
            )

            if (guidedTip != null) {
                Text(
                    text = (if (russian) "Наводка: " else "Guidance: ") + guidedTip,
                    color = MaterialTheme.colorScheme.primary,
                )
            }

            access.requirements.forEach { requirementAccess ->
                KnowledgeRequirementRow(
                    access = requirementAccess,
                    russian = russian,
                    onOpenNote = onOpenNote,
                    onOpenQuiz = onOpenQuiz,
                )
            }
        }
    }
}

@Composable
private fun KnowledgeRequirementRow(
    access: KnowledgeRequirementAccess,
    russian: Boolean,
    onOpenNote: (String, String) -> Unit,
    onOpenQuiz: (String, String) -> Unit,
) {
    val requirement = access.requirement
    val progress = access.progress
    val title = progress?.title ?: if (russian) requirement.fallbackTitleRu else requirement.fallbackTitleEn
    val stateLabel = when {
        access.passed -> if (russian) "Пройдено" else "Mastered"
        progress?.started == true -> if (russian) "Начато" else "Discovered"
        access.purchased -> if (russian) "Куплен доступ" else "Access purchased"
        else -> if (russian) "Не начато" else "Not started"
    }

    Column(
        modifier = Modifier.fillMaxWidth(),
        verticalArrangement = Arrangement.spacedBy(6.dp),
    ) {
        Text(
            text = "${requirement.courseCode.uppercase()} · $title",
            fontWeight = FontWeight.SemiBold,
        )
        Text(
            text = buildString {
                append(stateLabel)
                if (progress != null && progress.completedAttempts > 0) {
                    append(" · ")
                    append(if (russian) "попыток: " else "attempts: ")
                    append(progress.completedAttempts)
                    append(" · ")
                    append("%.1f%%".format(progress.bestAccuracyPercent))
                }
            },
            style = MaterialTheme.typography.bodySmall,
            color = MaterialTheme.colorScheme.onSurfaceVariant,
        )
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            OutlinedButton(
                onClick = { onOpenNote(requirement.courseCode, requirement.topicSlug) },
                enabled = access.notesAccessible,
                modifier = Modifier.weight(1f),
            ) {
                Text(if (russian) "Конспект" else "Notes")
            }
            OutlinedButton(
                onClick = { onOpenQuiz(requirement.courseCode, requirement.topicSlug) },
                enabled = access.testAccessible,
                modifier = Modifier.weight(1f),
            ) {
                Text(if (russian) "Тест" else "Quiz")
            }
        }
    }
}

private fun gateDescription(
    state: KnowledgeGateState,
    russian: Boolean,
): String = when (state) {
    KnowledgeGateState.LOCKED -> if (russian) {
        "Требуемые темы ещё не начаты. Инцидент доступен как ознакомительный сценарий с Guided Mode."
    } else {
        "Required topics have not been started yet. The incident remains available as a guided discovery scenario."
    }

    KnowledgeGateState.DISCOVERED -> if (russian) {
        "Часть требуемого материала уже изучается. Guided Mode останется включён до прохождения всех тем."
    } else {
        "Some prerequisite material has been discovered. Guided Mode stays enabled until all required topics are mastered."
    }

    KnowledgeGateState.MASTERED -> if (russian) {
        "Все связанные темы пройдены. Сценарий работает в Normal Mode без команд-подсказок."
    } else {
        "All related topics are mastered. The scenario runs in Normal Mode without command suggestion chips."
    }
}

@Composable
private fun IncidentEvaluationCard(
    evaluation: IncidentEvaluation,
    russian: Boolean,
) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            Text(
                text = if (russian) {
                    "Результат: ${incidentTitle(evaluation.incidentKind, true)}"
                } else {
                    "Result: ${evaluation.title}"
                },
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )
            ValueRow(if (russian) "Диагностика" else "Diagnosis", evaluation.diagnosis.name)
            ValueRow(if (russian) "Безопасность" else "Safety", evaluation.safety.name)
            ValueRow(if (russian) "Время" else "Time", evaluation.time.name)
            ValueRow(if (russian) "Подсказки" else "Hints used", evaluation.hintsUsed.toString())
            ValueRow(
                if (russian) "Затрачено" else "Elapsed",
                if (russian) "${evaluation.elapsedMinutes} мин" else "${evaluation.elapsedMinutes} min",
            )
            ValueRow(if (russian) "Итог" else "Score", "${evaluation.score}/100")
        }
    }
}

@Composable
private fun InfrastructureNodeCard(
    node: InfrastructureNode,
    russian: Boolean,
) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
            ) {
                Text(
                    text = node.name,
                    style = MaterialTheme.typography.titleLarge,
                    fontWeight = FontWeight.SemiBold,
                )
                Text(
                    text = statusLabel(node.status, russian),
                    style = MaterialTheme.typography.labelLarge,
                )
            }

            MetricRow("CPU", node.metrics.cpuPercent)
            MetricRow("RAM", node.metrics.ramPercent)
            MetricRow(if (russian) "Диск" else "Disk", node.metrics.diskPercent)
        }
    }
}

@Composable
private fun MetricRow(label: String, percent: Int) {
    Column(verticalArrangement = Arrangement.spacedBy(4.dp)) {
        Row(
            modifier = Modifier.fillMaxWidth(),
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Text(label)
            Text("$percent%")
        }
        LinearProgressIndicator(
            progress = { percent.coerceIn(0, 100) / 100f },
            modifier = Modifier.fillMaxWidth(),
        )
    }
}

private fun statusLabel(
    status: InfrastructureStatus,
    russian: Boolean,
): String = when (status) {
    InfrastructureStatus.HEALTHY -> if (russian) "Работает" else "Healthy"
    InfrastructureStatus.WARNING -> if (russian) "Внимание" else "Warning"
    InfrastructureStatus.DOWN -> if (russian) "Недоступен" else "Down"
}

@Composable
private fun WorldProgressCard(
    world: WorldState,
    russian: Boolean,
) {
    val nextRank = nextWorldRank(world.player.worldRank)

    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(10.dp),
        ) {
            Text(
                text = if (russian) "Прогресс компании" else "Company progress",
                style = MaterialTheme.typography.titleMedium,
                fontWeight = FontWeight.SemiBold,
            )
            ValueRow(
                label = if (russian) "Баланс компании" else "Company balance",
                value = "€${world.company.balance}",
            )
            ValueRow(
                label = if (russian) "Кредиты инженера" else "Engineer Credits",
                value = "${world.player.credits} CR",
            )
            ValueRow(
                label = if (russian) "Ранг DBA World" else "DBA World rank",
                value = rankLabel(world.player.worldRank, russian),
            )
            ValueRow(
                label = if (russian) "Решено инцидентов" else "Incidents resolved",
                value = "${world.completedIncidents.size}/3",
            )

            if (nextRank != null) {
                val currentFloor = world.player.worldRank.minimumXp
                val range = (nextRank.minimumXp - currentFloor).coerceAtLeast(1)
                val earned = (world.player.xp - currentFloor).coerceIn(0, range)

                Row(
                    modifier = Modifier.fillMaxWidth(),
                    horizontalArrangement = Arrangement.SpaceBetween,
                ) {
                    Text("XP")
                    Text("${world.player.xp} / ${nextRank.minimumXp}")
                }
                LinearProgressIndicator(
                    progress = { earned.toFloat() / range.toFloat() },
                    modifier = Modifier.fillMaxWidth(),
                )
                Text(
                    text = if (russian) {
                        "Следующий ранг: ${rankLabel(nextRank, true)}"
                    } else {
                        "Next rank: ${rankLabel(nextRank, false)}"
                    },
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            } else {
                ValueRow(label = "XP", value = world.player.xp.toString())
            }
        }
    }
}

@Composable
private fun ValueRow(
    label: String,
    value: String,
) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        horizontalArrangement = Arrangement.SpaceBetween,
    ) {
        Text(label)
        Text(value, fontWeight = FontWeight.SemiBold)
    }
}

private fun rankLabel(
    rank: WorldRank,
    russian: Boolean,
): String = when (rank) {
    WorldRank.TRAINEE_OPERATOR -> if (russian) "Стажёр-оператор" else "Trainee Operator"
    WorldRank.JUNIOR_OPERATOR -> if (russian) "Младший оператор" else "Junior Operator"
    WorldRank.DB_ENGINEER -> "DB Engineer"
    WorldRank.PRODUCTION_DBA -> "Production DBA"
    WorldRank.SENIOR_DBA -> "Senior DBA"
    WorldRank.INCIDENT_COMMANDER -> "Incident Commander"
    WorldRank.INFRASTRUCTURE_ARCHITECT -> "Infrastructure Architect"
}

private fun incidentTitle(kind: IncidentKind, russian: Boolean): String = when (kind) {
    IncidentKind.DISK_FULL -> if (russian) "Диск заполнен" else "Disk Full"
    IncidentKind.SERVICE_DOWN -> if (russian) "PostgreSQL не запускается" else "PostgreSQL Service Down"
    IncidentKind.HIGH_CPU_BAD_QUERY -> if (russian) "Высокий CPU / плохой запрос" else "High CPU / Bad Query"
}

private fun severityLabel(
    severity: IncidentSeverity,
    russian: Boolean,
): String = when (severity) {
    IncidentSeverity.MEDIUM -> if (russian) "Средний" else "Medium"
    IncidentSeverity.HIGH -> if (russian) "Высокий" else "High"
    IncidentSeverity.CRITICAL -> if (russian) "Критический" else "Critical"
}

private fun clockLabel(clock: WorldClock, russian: Boolean): String {
    val hh = clock.hour.toString().padStart(2, '0')
    val mm = clock.minute.toString().padStart(2, '0')
    val date = clock.date
    return if (russian) {
        val month = listOf(
            "января",
            "февраля",
            "марта",
            "апреля",
            "мая",
            "июня",
            "июля",
            "августа",
            "сентября",
            "октября",
            "ноября",
            "декабря",
        )[date.month - 1]
        "${date.dayOfMonth} $month ${date.year} · $hh:$mm"
    } else {
        val month = date.month.toString().padStart(2, '0')
        val day = date.dayOfMonth.toString().padStart(2, '0')
        "$day.$month.${date.year} · $hh:$mm"
    }
}
