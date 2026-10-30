package dev.dbatrainer.app.game.ui

import androidx.compose.animation.core.LinearEasing
import androidx.compose.animation.core.RepeatMode
import androidx.compose.animation.core.animateFloat
import androidx.compose.animation.core.infiniteRepeatable
import androidx.compose.animation.core.rememberInfiniteTransition
import androidx.compose.animation.core.tween
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.rememberScrollState
import androidx.compose.foundation.verticalScroll
import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Box
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.Row
import androidx.compose.foundation.layout.Spacer
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.height
import androidx.compose.foundation.layout.offset
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.layout.size
import androidx.compose.foundation.layout.width
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.Card
import androidx.compose.material3.LinearProgressIndicator
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.OutlinedButton
import androidx.compose.material3.Surface
import androidx.compose.material3.Text
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.geometry.Offset
import androidx.compose.ui.geometry.Size
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.Path
import androidx.compose.ui.graphics.graphicsLayer
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import dev.dbatrainer.app.game.DbaWorldController
import dev.dbatrainer.app.game.domain.IncidentSeverity
import dev.dbatrainer.app.game.domain.InfrastructureNode
import dev.dbatrainer.app.game.domain.InfrastructureStatus
import dev.dbatrainer.app.game.domain.WorkEventType
import dev.dbatrainer.app.game.domain.WorldEvent
import dev.dbatrainer.app.game.domain.WorldClock
import dev.dbatrainer.app.game.domain.WorldState
import dev.dbatrainer.app.game.engine.WorldDayPhase
import dev.dbatrainer.app.game.engine.WorldEnvironment
import dev.dbatrainer.app.game.engine.WorldEnvironmentEngine
import dev.dbatrainer.app.game.engine.WorldSeason
import dev.dbatrainer.app.game.engine.WorldWeather
import dev.dbatrainer.app.game.engine.WorldEventEngine
import dev.dbatrainer.app.game.engine.WorkdayEngine

@Composable
internal fun DbaWorldOutdoorScreen(
    controller: DbaWorldController,
    russian: Boolean,
) {
    val world = controller.world ?: return
    val environment = WorldEnvironmentEngine.environmentFor(world.clock)
    val transition = rememberInfiniteTransition(label = "outdoor-weather")
    val weatherOffset by transition.animateFloat(
        initialValue = 0f,
        targetValue = 1f,
        animationSpec = infiniteRepeatable(
            animation = tween(2200, easing = LinearEasing),
            repeatMode = RepeatMode.Restart,
        ),
        label = "weather-offset",
    )

    Box(modifier = Modifier.fillMaxSize()) {
        OutdoorWorldCanvas(
            world = world,
            environment = environment,
            weatherOffset = weatherOffset,
            modifier = Modifier.fillMaxSize(),
        )

        OutdoorHud(
            world = world,
            environment = environment,
            russian = russian,
            modifier = Modifier
                .align(Alignment.TopCenter)
                .padding(12.dp),
        )

        Box(
            modifier = Modifier
                .align(Alignment.Center)
                .offset(y = 30.dp)
                .fillMaxWidth(0.78f)
                .height(290.dp)
                .clickable(onClick = controller::enterBuilding),
        )

        if (WorldEventEngine.unresolvedAlertCount(world) > 0) {
            WorldAlertBadge(
                critical = world.incident?.severity == IncidentSeverity.CRITICAL,
                modifier = Modifier
                    .align(Alignment.Center)
                    .offset(x = 108.dp, y = (-34).dp),
            )
        }

        Surface(
            modifier = Modifier
                .align(Alignment.BottomCenter)
                .padding(18.dp)
                .clickable(onClick = controller::enterBuilding),
            shape = RoundedCornerShape(18.dp),
            color = MaterialTheme.colorScheme.surface.copy(alpha = 0.94f),
            shadowElevation = 10.dp,
            tonalElevation = 5.dp,
        ) {
            Column(
                modifier = Modifier.padding(horizontal = 22.dp, vertical = 12.dp),
                horizontalAlignment = Alignment.CenterHorizontally,
            ) {
                Text(
                    text = if (russian) "DBA World Labs" else "DBA World Labs",
                    fontWeight = FontWeight.Bold,
                )
                Text(
                    text = if (russian) "Нажмите, чтобы войти в здание" else "Tap to enter the building",
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun OutdoorWorldCanvas(
    world: WorldState,
    environment: WorldEnvironment,
    weatherOffset: Float,
    modifier: Modifier = Modifier,
) {
    Canvas(modifier = modifier) {
        val sky = outdoorSky(environment.dayPhase)
        drawRect(
            brush = Brush.verticalGradient(sky),
            size = size,
        )

        val horizon = size.height * 0.58f
        val lawnColor = when (environment.season) {
            WorldSeason.WINTER -> Color(0xFFDDE7EE)
            WorldSeason.SPRING -> Color(0xFF5F7E58)
            WorldSeason.SUMMER -> Color(0xFF4F7346)
            WorldSeason.AUTUMN -> Color(0xFF665741)
        }
        drawRect(
            color = lawnColor,
            topLeft = Offset(0f, horizon),
            size = Size(size.width, size.height - horizon),
        )

        if (environment.weather == WorldWeather.CLOUDY || environment.weather == WorldWeather.RAIN || environment.weather == WorldWeather.SNOW) {
            repeat(4) { i ->
                drawCloud(
                    center = Offset(size.width * (0.18f + i * 0.22f), size.height * (0.14f + (i % 2) * 0.035f)),
                    scale = 0.9f + i * 0.12f,
                    alpha = if (environment.weather == WorldWeather.CLOUDY) 0.72f else 0.58f,
                )
            }
        }

        val campusLeft = size.width * 0.08f
        val campusRight = size.width * 0.92f
        val sidewalkTop = size.height * 0.70f
        drawRect(
            color = Color(0xFF8D9299),
            topLeft = Offset(campusLeft, sidewalkTop),
            size = Size(campusRight - campusLeft, size.height * 0.06f),
        )
        drawLine(
            color = Color.White.copy(alpha = 0.18f),
            start = Offset(campusLeft, sidewalkTop + 6f),
            end = Offset(campusRight, sidewalkTop + 6f),
            strokeWidth = 2f,
        )

        val roadTop = size.height * 0.79f
        drawRect(
            color = if (environment.weather == WorldWeather.RAIN) Color(0xFF232831) else Color(0xFF303238),
            topLeft = Offset(0f, roadTop),
            size = Size(size.width, size.height - roadTop),
        )
        drawLine(
            color = Color(0xFFCFB95C),
            start = Offset(size.width * 0.5f, roadTop + 10f),
            end = Offset(size.width * 0.5f, size.height),
            strokeWidth = 5f,
        )
        if (environment.weather == WorldWeather.RAIN) {
            drawRect(
                color = Color.White.copy(alpha = 0.06f),
                topLeft = Offset(size.width * 0.18f, roadTop + 14f),
                size = Size(size.width * 0.64f, 24f),
            )
        }

        val buildingLeft = size.width * 0.16f
        val buildingTop = size.height * 0.26f
        val buildingWidth = size.width * 0.68f
        val buildingHeight = size.height * 0.43f
        val annexWidth = buildingWidth * 0.22f
        val buildingColor = when (environment.dayPhase) {
            WorldDayPhase.NIGHT -> Color(0xFF323842)
            WorldDayPhase.EVENING -> Color(0xFF565D67)
            else -> Color(0xFF676F79)
        }
        drawRoundRect(
            color = buildingColor,
            topLeft = Offset(buildingLeft, buildingTop),
            size = Size(buildingWidth, buildingHeight),
            cornerRadius = androidx.compose.ui.geometry.CornerRadius(18f, 18f),
        )
        drawRoundRect(
            color = buildingColor.copy(alpha = 0.92f),
            topLeft = Offset(buildingLeft + buildingWidth - annexWidth, buildingTop + 26f),
            size = Size(annexWidth, buildingHeight - 26f),
            cornerRadius = androidx.compose.ui.geometry.CornerRadius(14f, 14f),
        )
        drawRect(
            color = Color(0xFF262C33),
            topLeft = Offset(buildingLeft - 10f, buildingTop - 18f),
            size = Size(buildingWidth + 20f, 24f),
        )

        val signWidth = buildingWidth * 0.42f
        val signLeft = buildingLeft + buildingWidth * 0.29f
        drawRoundRect(
            color = Color(0xFF15222B),
            topLeft = Offset(signLeft, buildingTop + 14f),
            size = Size(signWidth, 34f),
            cornerRadius = androidx.compose.ui.geometry.CornerRadius(8f, 8f),
        )
        repeat(9) { i ->
            val x = signLeft + 18f + i * ((signWidth - 36f) / 8f)
            drawCircle(
                color = Color(0xFF87D7FF).copy(alpha = if (i % 2 == 0) 0.95f else 0.65f),
                radius = 2.2f,
                center = Offset(x, buildingTop + 31f),
            )
        }

        val windowLit = environment.dayPhase != WorldDayPhase.DAY
        val windowColor = if (windowLit) Color(0xFFFFD36A) else Color(0xFF93BED4)
        repeat(3) { row ->
            repeat(5) { column ->
                val x = buildingLeft + 26f + column * ((buildingWidth - annexWidth - 84f) / 4f)
                val y = buildingTop + 62f + row * 63f
                drawRoundRect(
                    color = windowColor.copy(alpha = if ((row + column) % 4 == 0) 0.55f else 0.9f),
                    topLeft = Offset(x, y),
                    size = Size(36f, 30f),
                    cornerRadius = androidx.compose.ui.geometry.CornerRadius(4f, 4f),
                )
            }
        }
        repeat(4) { row ->
            val x = buildingLeft + buildingWidth - annexWidth + 20f
            val y = buildingTop + 52f + row * 47f
            drawRoundRect(
                color = windowColor.copy(alpha = if (row % 2 == 0) 0.85f else 0.6f),
                topLeft = Offset(x, y),
                size = Size(42f, 24f),
                cornerRadius = androidx.compose.ui.geometry.CornerRadius(4f, 4f),
            )
        }

        val doorWidth = buildingWidth * 0.18f
        val doorHeight = buildingHeight * 0.32f
        val doorLeft = buildingLeft + buildingWidth / 2f - doorWidth / 2f
        val doorTop = buildingTop + buildingHeight - doorHeight
        drawRoundRect(
            color = Color(0xFF1B2025),
            topLeft = Offset(doorLeft, doorTop),
            size = Size(doorWidth, doorHeight),
            cornerRadius = androidx.compose.ui.geometry.CornerRadius(10f, 10f),
        )
        drawLine(
            color = Color.White.copy(alpha = 0.12f),
            start = Offset(doorLeft + doorWidth / 2f, doorTop + 8f),
            end = Offset(doorLeft + doorWidth / 2f, doorTop + doorHeight - 8f),
            strokeWidth = 1.5f,
        )
        drawCircle(
            color = Color(0xFFE7C56B),
            radius = 4f,
            center = Offset(
                doorLeft + doorWidth * 0.74f,
                doorTop + doorHeight * 0.48f,
            ),
        )

        val lampXs = listOf(size.width * 0.1f, size.width * 0.9f)
        lampXs.forEach { lampX ->
            drawRect(
                color = Color(0xFF22272D),
                topLeft = Offset(lampX, horizon - 96f),
                size = Size(8f, 128f),
            )
            val lit = environment.dayPhase != WorldDayPhase.DAY
            if (lit) {
                drawCircle(
                    color = Color(0xFFFFE29A).copy(alpha = 0.18f),
                    radius = 40f,
                    center = Offset(lampX + 4f, horizon - 99f),
                )
            }
            drawCircle(
                color = if (lit) Color(0xFFFFDE82) else Color(0xFFAFB6BC),
                radius = 15f,
                center = Offset(lampX + 4f, horizon - 99f),
            )
        }

        val treeColor = when (environment.season) {
            WorldSeason.WINTER -> Color(0xFFBDC9D2)
            WorldSeason.SPRING -> Color(0xFF4A7E4D)
            WorldSeason.SUMMER -> Color(0xFF35683A)
            WorldSeason.AUTUMN -> Color(0xFF93683A)
        }
        drawTree(Offset(size.width * 0.08f, sidewalkTop + 6f), treeColor)
        drawTree(Offset(size.width * 0.88f, sidewalkTop + 6f), treeColor)

        if (environment.newYearDecorations) {
            repeat(12) { i ->
                val x = buildingLeft + 18f + i * ((buildingWidth - 36f) / 11f)
                val y = buildingTop - 4f + if (i % 2 == 0) 5f else 12f
                val color = when (i % 3) {
                    0 -> Color(0xFFE65353)
                    1 -> Color(0xFF58C984)
                    else -> Color(0xFFFFD45E)
                }
                drawCircle(color = color, radius = 5f, center = Offset(x, y))
            }
            val treeBase = Offset(buildingLeft - 42f, buildingTop + buildingHeight)
            val tree = Path().apply {
                moveTo(treeBase.x, treeBase.y - 112f)
                lineTo(treeBase.x - 45f, treeBase.y - 10f)
                lineTo(treeBase.x + 45f, treeBase.y - 10f)
                close()
            }
            drawPath(tree, Color(0xFF2E6B45))
            drawRect(
                Color(0xFF65442E),
                topLeft = Offset(treeBase.x - 7f, treeBase.y - 14f),
                size = Size(14f, 24f),
            )
        }

        drawWeather(
            weather = environment.weather,
            phase = weatherOffset,
            seed = world.clock.day,
            horizon = horizon,
        )
    }
}

private fun androidx.compose.ui.graphics.drawscope.DrawScope.drawCloud(
    center: Offset,
    scale: Float,
    alpha: Float,
) {
    val color = Color.White.copy(alpha = alpha)
    drawCircle(color = color, radius = 28f * scale, center = center)
    drawCircle(color = color, radius = 22f * scale, center = Offset(center.x - 26f * scale, center.y + 6f * scale))
    drawCircle(color = color, radius = 24f * scale, center = Offset(center.x + 26f * scale, center.y + 8f * scale))
    drawRoundRect(
        color = color,
        topLeft = Offset(center.x - 42f * scale, center.y + 8f * scale),
        size = Size(84f * scale, 24f * scale),
        cornerRadius = androidx.compose.ui.geometry.CornerRadius(16f * scale, 16f * scale),
    )
}

private fun androidx.compose.ui.graphics.drawscope.DrawScope.drawTree(
    base: Offset,
    foliage: Color,
) {
    drawRect(
        color = Color(0xFF5C4431),
        topLeft = Offset(base.x - 5f, base.y - 48f),
        size = Size(10f, 48f),
    )
    drawCircle(foliage, radius = 24f, center = Offset(base.x, base.y - 62f))
    drawCircle(foliage.copy(alpha = 0.88f), radius = 20f, center = Offset(base.x - 16f, base.y - 54f))
    drawCircle(foliage.copy(alpha = 0.88f), radius = 20f, center = Offset(base.x + 16f, base.y - 52f))
}

private fun androidx.compose.ui.graphics.drawscope.DrawScope.drawWeather(
    weather: WorldWeather,
    phase: Float,
    seed: Int,
    horizon: Float,
) {
    when (weather) {
        WorldWeather.SNOW -> repeat(46) { i ->
            val x = ((i * 73 + seed * 19) % 997) / 997f * size.width
            val base = ((i * 137 + seed * 31) % 991) / 991f
            val y = ((base + phase) % 1f) * (size.height * 0.82f)
            drawCircle(
                color = Color.White.copy(alpha = 0.82f),
                radius = 2.2f + (i % 3),
                center = Offset(x, y),
            )
        }

        WorldWeather.RAIN -> repeat(54) { i ->
            val x = ((i * 61 + seed * 23) % 991) / 991f * size.width
            val base = ((i * 127 + seed * 41) % 983) / 983f
            val y = ((base + phase) % 1f) * horizon
            drawLine(
                color = Color(0xFFB7D4E8).copy(alpha = 0.55f),
                start = Offset(x, y),
                end = Offset(x - 6f, y + 18f),
                strokeWidth = 2f,
            )
        }

        WorldWeather.CLOUDY,
        WorldWeather.CLEAR -> Unit
    }
}

@Composable
private fun OutdoorHud(
    world: WorldState,
    environment: WorldEnvironment,
    russian: Boolean,
    modifier: Modifier = Modifier,
) {
    Surface(
        modifier = modifier.fillMaxWidth(),
        shape = RoundedCornerShape(16.dp),
        color = MaterialTheme.colorScheme.surface.copy(alpha = 0.9f),
        tonalElevation = 5.dp,
    ) {
        Row(
            modifier = Modifier.padding(horizontal = 14.dp, vertical = 10.dp),
            horizontalArrangement = Arrangement.SpaceBetween,
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Column {
                Text(
                    text = world.company.name,
                    fontWeight = FontWeight.Bold,
                    style = MaterialTheme.typography.titleMedium,
                )
                Text(
                    text = shellClockLabel(world.clock),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            Column(horizontalAlignment = Alignment.End) {
                Text(
                    text = weatherLabel(environment, russian),
                    style = MaterialTheme.typography.labelMedium,
                )
                Text(
                    text = "€${world.company.balance} · ${world.player.credits} CR",
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
internal fun DbaWorldWorkstationScreen(
    controller: DbaWorldController,
    russian: Boolean,
) {
    val world = controller.world ?: return
    val db = world.nodes.firstOrNull { it.id == "db-primary" }
    val environment = WorldEnvironmentEngine.environmentFor(world.clock)
    val events = WorldEventEngine.eventsFor(world)
    val activeAlerts = events.count { it.type != WorkEventType.TASK }
    val pendingTasks = events.count { it.type == WorkEventType.TASK }

    Box(modifier = Modifier.fillMaxSize()) {
        Canvas(modifier = Modifier.fillMaxSize()) {
            val topWall = if (environment.dayPhase == WorldDayPhase.NIGHT) Color(0xFF31363B) else Color(0xFF4A5157)
            val bottomWall = if (environment.dayPhase == WorldDayPhase.NIGHT) Color(0xFF272B2F) else Color(0xFF3A4248)
            drawRect(
                brush = Brush.verticalGradient(listOf(topWall, bottomWall)),
                size = size,
            )
            drawRect(
                color = Color(0xFF24282D),
                topLeft = Offset(0f, size.height * 0.73f),
                size = Size(size.width, size.height * 0.27f),
            )
            drawRect(
                color = Color(0xFF7B573D),
                topLeft = Offset(size.width * 0.08f, size.height * 0.57f),
                size = Size(size.width * 0.7f, size.height * 0.13f),
            )
            drawRect(
                color = Color(0xFF4A3629),
                topLeft = Offset(size.width * 0.12f, size.height * 0.70f),
                size = Size(size.width * 0.05f, size.height * 0.18f),
            )
            drawRect(
                color = Color(0xFF4A3629),
                topLeft = Offset(size.width * 0.66f, size.height * 0.70f),
                size = Size(size.width * 0.05f, size.height * 0.18f),
            )
            drawRoundRect(
                color = Color(0xFF1A2128),
                topLeft = Offset(size.width * 0.16f, size.height * 0.49f),
                size = Size(size.width * 0.22f, size.height * 0.075f),
                cornerRadius = androidx.compose.ui.geometry.CornerRadius(12f, 12f),
            )
            drawRoundRect(
                color = Color(0xFF18222B),
                topLeft = Offset(size.width * 0.57f, size.height * 0.24f),
                size = Size(size.width * 0.22f, size.height * 0.15f),
                cornerRadius = androidx.compose.ui.geometry.CornerRadius(16f, 16f),
            )
            repeat(5) { i ->
                val width = size.width * (0.035f + i * 0.01f)
                val height = size.height * (0.02f + i * 0.008f)
                drawRect(
                    color = Color(0xFF78C6EF).copy(alpha = 0.26f + i * 0.05f),
                    topLeft = Offset(size.width * 0.595f, size.height * (0.285f + i * 0.018f)),
                    size = Size(width, height),
                )
            }
            drawRoundRect(
                color = Color(0xFFC6C6C6),
                topLeft = Offset(size.width * 0.57f, size.height * 0.59f),
                size = Size(size.width * 0.05f, size.height * 0.018f),
                cornerRadius = androidx.compose.ui.geometry.CornerRadius(6f, 6f),
            )
            drawRect(
                color = Color(0xFFD7F0FF).copy(alpha = 0.22f),
                topLeft = Offset(size.width * 0.22f, size.height * 0.25f),
                size = Size(size.width * 0.18f, size.height * 0.22f),
            )
        }

        Surface(
            modifier = Modifier
                .align(Alignment.Center)
                .offset(x = (-42).dp, y = 8.dp)
                .width(224.dp)
                .height(150.dp)
                .clickable(onClick = controller::openComputer),
            shape = RoundedCornerShape(12.dp),
            color = Color(0xFF121B24),
            shadowElevation = 14.dp,
        ) {
            Box(contentAlignment = Alignment.Center) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    Text(
                        text = "DBA WORLD",
                        color = Color(0xFF7CD9FF),
                        fontWeight = FontWeight.Bold,
                        fontFamily = FontFamily.Monospace,
                    )
                    Text(
                        text = if (russian) "Рабочая станция" else "Engineering workstation",
                        color = Color(0xFFD8E5EC),
                        style = MaterialTheme.typography.labelSmall,
                    )
                    Spacer(Modifier.height(8.dp))
                    Text(
                        text = if (world.incident != null) {
                            if (russian) "Monitoring: ALERT" else "Monitoring: ALERT"
                        } else {
                            if (russian) "Monitoring: stable" else "Monitoring: stable"
                        },
                        color = if (world.incident != null) Color(0xFFFF7A7A) else Color(0xFF78DCA0),
                        fontFamily = FontFamily.Monospace,
                        style = MaterialTheme.typography.labelSmall,
                    )
                }
                if (world.incident != null) {
                    WorldAlertBadge(
                        critical = world.incident.severity == IncidentSeverity.CRITICAL,
                        modifier = Modifier
                            .align(Alignment.TopEnd)
                            .padding(10.dp),
                    )
                }
            }
        }

        Surface(
            modifier = Modifier
                .align(Alignment.CenterEnd)
                .offset(x = (-14).dp, y = 26.dp)
                .width(98.dp)
                .height(238.dp)
                .clickable(onClick = controller::openServerRoom),
            color = Color(0xFF20262C),
            shape = RoundedCornerShape(topStart = 8.dp, topEnd = 8.dp),
            shadowElevation = 8.dp,
        ) {
            Box(contentAlignment = Alignment.Center) {
                Column(horizontalAlignment = Alignment.CenterHorizontally) {
                    Text("SERVER", color = Color.White, fontWeight = FontWeight.Bold)
                    Text("ROOM", color = Color.White, fontWeight = FontWeight.Bold)
                    Text(
                        text = if (russian) "стойки и узлы" else "racks & nodes",
                        color = Color(0xFFBCCAD5),
                        style = MaterialTheme.typography.labelSmall,
                    )
                    if (db?.status != InfrastructureStatus.HEALTHY) {
                        Spacer(Modifier.height(10.dp))
                        WorldAlertBadge(critical = db?.status == InfrastructureStatus.DOWN)
                    }
                }
            }
        }

        Surface(
            modifier = Modifier
                .align(Alignment.TopEnd)
                .padding(top = 24.dp, end = 26.dp)
                .width(130.dp)
                .height(94.dp),
            shape = RoundedCornerShape(12.dp),
            color = Color(0xFF121920),
            shadowElevation = 6.dp,
        ) {
            Column(
                modifier = Modifier.padding(12.dp),
                verticalArrangement = Arrangement.spacedBy(6.dp),
            ) {
                Text(
                    text = if (russian) "Стена NOC" else "NOC wall",
                    color = Color.White,
                    fontWeight = FontWeight.Bold,
                    style = MaterialTheme.typography.labelMedium,
                )
                Text(
                    text = if (activeAlerts > 0) {
                        if (russian) "alerts: $activeAlerts" else "alerts: $activeAlerts"
                    } else {
                        if (russian) "all systems normal" else "all systems normal"
                    },
                    color = if (activeAlerts > 0) Color(0xFFFF8B8B) else Color(0xFF8FE6B2),
                    style = MaterialTheme.typography.labelSmall,
                    fontFamily = FontFamily.Monospace,
                )
                Text(
                    text = if (russian) "tasks: $pendingTasks" else "tasks: $pendingTasks",
                    color = Color(0xFFFFD27A),
                    style = MaterialTheme.typography.labelSmall,
                    fontFamily = FontFamily.Monospace,
                )
                Text(
                    text = "db-primary ${db?.metrics?.diskPercent ?: 0}%",
                    color = Color(0xFF9AD5FF),
                    style = MaterialTheme.typography.labelSmall,
                    fontFamily = FontFamily.Monospace,
                )
            }
        }

        Surface(
            modifier = Modifier
                .align(Alignment.BottomStart)
                .padding(18.dp),
            shape = RoundedCornerShape(16.dp),
            color = MaterialTheme.colorScheme.surface.copy(alpha = 0.94f),
            tonalElevation = 4.dp,
        ) {
            Column(modifier = Modifier.padding(horizontal = 14.dp, vertical = 10.dp)) {
                Text(
                    text = if (russian) "Рабочее место инженера" else "Engineer workstation",
                    fontWeight = FontWeight.Bold,
                )
                Text(
                    text = if (russian) {
                        "Монитор — рабочий компьютер · дверь — серверная · верхний экран — мониторинг"
                    } else {
                        "Monitor — desktop · door — server room · upper screen — monitoring"
                    },
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }

        OutlinedButton(
            onClick = controller::goOutdoor,
            modifier = Modifier
                .align(Alignment.TopStart)
                .padding(12.dp),
        ) {
            Text(if (russian) "← Улица" else "← Outside")
        }
    }
}
@Composable
internal fun DbaWorldComputerScreen(
    world: WorldState,
    russian: Boolean,
    onOpenTerminal: () -> Unit,
    onOpenMonitoring: () -> Unit,
    onOpenLearningCenter: () -> Unit,
    onOpenServerRoom: () -> Unit,
    onCloseComputer: () -> Unit,
    onCloseShift: () -> Unit,
) {
    val events = WorldEventEngine.eventsFor(world)

    Column(
        modifier = Modifier
            .fillMaxSize()
            .background(Color(0xFF101820)),
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .background(Color(0xFF1E2B35))
                .padding(horizontal = 14.dp, vertical = 10.dp),
            verticalAlignment = Alignment.CenterVertically,
            horizontalArrangement = Arrangement.SpaceBetween,
        ) {
            Text(
                text = if (russian) "Рабочий компьютер" else "Engineering workstation",
                color = Color.White,
                fontWeight = FontWeight.Bold,
            )
            Text(
                text = shellClockLabel(world.clock),
                color = Color(0xFFB8CCD9),
                style = MaterialTheme.typography.labelSmall,
            )
        }

        Column(
            modifier = Modifier
                .fillMaxSize()
                .verticalScroll(rememberScrollState())
                .padding(18.dp),
            verticalArrangement = Arrangement.spacedBy(14.dp),
        ) {
            Text(
                text = if (russian) "Инженерные инструменты" else "Engineering tools",
                color = Color(0xFFDCEAF1),
                style = MaterialTheme.typography.titleLarge,
                fontWeight = FontWeight.Bold,
            )

            WorkShiftCard(
                world = world,
                russian = russian,
                onCloseShift = onCloseShift,
            )

            if (events.isNotEmpty()) {
                Text(
                    text = if (russian) "События смены" else "Shift events",
                    color = Color(0xFFDCEAF1),
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.Bold,
                )
                events.take(3).forEach { event ->
                    WorkEventCompactRow(event = event, russian = russian)
                }
            }

            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
            ) {
                DesktopAppTile(
                    title = if (russian) "Терминал" else "Terminal",
                    subtitle = "Linux / PostgreSQL",
                    badge = world.incident != null,
                    onClick = onOpenTerminal,
                    modifier = Modifier.weight(1f),
                )
                DesktopAppTile(
                    title = if (russian) "Мониторинг" else "Monitoring",
                    subtitle = if (russian) "Сервисы и метрики" else "Services and metrics",
                    badge = world.incident != null,
                    onClick = onOpenMonitoring,
                    modifier = Modifier.weight(1f),
                )
            }

            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.spacedBy(12.dp),
            ) {
                DesktopAppTile(
                    title = if (russian) "Учебный центр" else "Learning Center",
                    subtitle = "${world.player.credits} CR",
                    badge = false,
                    onClick = onOpenLearningCenter,
                    modifier = Modifier.weight(1f),
                )
                DesktopAppTile(
                    title = if (russian) "Серверная" else "Server Room",
                    subtitle = if (russian) "Физическая инфраструктура" else "Physical infrastructure",
                    badge = world.incident != null,
                    onClick = onOpenServerRoom,
                    modifier = Modifier.weight(1f),
                )
            }

            Spacer(modifier = Modifier.height(4.dp))

            OutlinedButton(
                onClick = onCloseComputer,
                modifier = Modifier.fillMaxWidth(),
            ) {
                Text(if (russian) "Закрыть компьютер" else "Leave workstation computer")
            }
        }
    }
}

@Composable
private fun WorkShiftCard(
    world: WorldState,
    russian: Boolean,
    onCloseShift: () -> Unit,
) {
    val canClose = WorkdayEngine.canCloseShift(world)
    val completed = world.workday.completedTasks.size
    Card(modifier = Modifier.fillMaxWidth()) {
        Column(
            modifier = Modifier.padding(12.dp),
            verticalArrangement = Arrangement.spacedBy(6.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Text(
                    text = if (russian) "Рабочая смена" else "Work shift",
                    fontWeight = FontWeight.Bold,
                )
                Text(
                    text = if (russian) WorkdayEngine.shiftLabelRu(world) else WorkdayEngine.shiftLabelEn(world),
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.primary,
                )
            }
            Text(
                text = if (russian) {
                    "Задачи: $completed/${WorkdayEngine.DAILY_TASK_COUNT} · инцидентов решено: ${world.workday.incidentsResolved}"
                } else {
                    "Tasks: $completed/${WorkdayEngine.DAILY_TASK_COUNT} · incidents resolved: ${world.workday.incidentsResolved}"
                },
                style = MaterialTheme.typography.bodySmall,
            )
            if (canClose) {
                OutlinedButton(
                    onClick = onCloseShift,
                    modifier = Modifier.fillMaxWidth(),
                ) {
                    Text(if (russian) "Закрыть смену" else "Close shift")
                }
            }
            world.lastShiftReport?.takeIf { it.day == world.workday.day || world.workday.shiftClosed }?.let { report ->
                Text(
                    text = if (russian) {
                        "Отчёт: задачи ${report.tasksCompleted}/${report.tasksTotal}, инциденты ${report.incidentsResolved}"
                    } else {
                        "Report: tasks ${report.tasksCompleted}/${report.tasksTotal}, incidents ${report.incidentsResolved}"
                    },
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            world.lastMonthlyReport?.let { report ->
                Text(
                    text = if (russian) {
                        "Месяц ${report.month.toString().padStart(2, '0')}.${report.year}: ${signedMoney(report.netChange)} · зарплата +${report.playerSalaryCredits} CR"
                    } else {
                        "Month ${report.month.toString().padStart(2, '0')}.${report.year}: ${signedMoney(report.netChange)} · salary +${report.playerSalaryCredits} CR"
                    },
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun WorkEventCompactRow(
    event: WorldEvent,
    russian: Boolean,
) {
    val color = workEventColor(event.type)
    Surface(
        modifier = Modifier.fillMaxWidth(),
        shape = RoundedCornerShape(10.dp),
        color = color.copy(alpha = 0.12f),
    ) {
        Row(
            modifier = Modifier.padding(horizontal = 10.dp, vertical = 8.dp),
            horizontalArrangement = Arrangement.spacedBy(9.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Box(
                modifier = Modifier
                    .size(10.dp)
                    .background(color, CircleShape),
            )
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = if (russian) event.titleRu else event.titleEn,
                    fontWeight = FontWeight.SemiBold,
                    style = MaterialTheme.typography.bodySmall,
                )
                Text(
                    text = "${workEventTypeLabel(event.type, russian)} · ${workEventSourceLabel(event, russian)}",
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

@Composable
private fun MonitoringEventCard(
    event: WorldEvent,
    russian: Boolean,
) {
    val color = workEventColor(event.type)
    Surface(
        modifier = Modifier.fillMaxWidth(),
        shape = RoundedCornerShape(14.dp),
        color = color.copy(alpha = 0.12f),
    ) {
        Row(
            modifier = Modifier.padding(14.dp),
            horizontalArrangement = Arrangement.spacedBy(10.dp),
            verticalAlignment = Alignment.Top,
        ) {
            if (event.type == WorkEventType.INCIDENT || event.type == WorkEventType.WARNING) {
                WorldAlertBadge(critical = event.critical)
            } else {
                Box(
                    modifier = Modifier
                        .size(30.dp)
                        .background(color, CircleShape),
                    contentAlignment = Alignment.Center,
                ) {
                    Text("•", color = Color.White, fontWeight = FontWeight.Black)
                }
            }
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = if (russian) event.titleRu else event.titleEn,
                    fontWeight = FontWeight.Bold,
                )
                Text(
                    text = "${workEventTypeLabel(event.type, russian)} · ${workEventSourceLabel(event, russian)}",
                    style = MaterialTheme.typography.labelSmall,
                    color = color,
                )
                Text(
                    text = if (russian) event.descriptionRu else event.descriptionEn,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
        }
    }
}

private fun workEventColor(type: WorkEventType): Color = when (type) {
    WorkEventType.TASK -> Color(0xFF4D8FD6)
    WorkEventType.WARNING -> Color(0xFFD39B2D)
    WorkEventType.INCIDENT -> Color(0xFFD64A4A)
}

private fun workEventTypeLabel(type: WorkEventType, russian: Boolean): String = when (type) {
    WorkEventType.TASK -> if (russian) "Задача" else "Task"
    WorkEventType.WARNING -> if (russian) "Предупреждение" else "Warning"
    WorkEventType.INCIDENT -> if (russian) "Инцидент" else "Incident"
}

private fun workEventSourceLabel(event: WorldEvent, russian: Boolean): String = when (event.source) {
    dev.dbatrainer.app.game.domain.WorkEventSource.WORKSTATION -> if (russian) "рабочее место" else "workstation"
    dev.dbatrainer.app.game.domain.WorkEventSource.MONITORING -> if (russian) "monitoring" else "monitoring"
    dev.dbatrainer.app.game.domain.WorkEventSource.APPLICATION -> event.targetNodeId ?: "application"
    dev.dbatrainer.app.game.domain.WorkEventSource.SERVER -> event.targetNodeId ?: "server"
    dev.dbatrainer.app.game.domain.WorkEventSource.POSTGRESQL -> event.targetNodeId ?: "PostgreSQL"
    dev.dbatrainer.app.game.domain.WorkEventSource.NETWORK -> if (russian) "сеть" else "network"
    dev.dbatrainer.app.game.domain.WorkEventSource.BACKUP -> if (russian) "backup" else "backup"
}

private fun signedMoney(value: Long): String = if (value >= 0) "+€$value" else "-€${-value}"

@Composable
private fun DesktopAppTile(
    title: String,
    subtitle: String,
    badge: Boolean,
    onClick: () -> Unit,
    modifier: Modifier = Modifier,
) {
    Card(
        modifier = modifier
            .height(132.dp)
            .clickable(onClick = onClick),
    ) {
        Box(modifier = Modifier.fillMaxSize()) {
            Column(
                modifier = Modifier.padding(14.dp),
                verticalArrangement = Arrangement.spacedBy(8.dp),
            ) {
                Box(
                    modifier = Modifier
                        .size(42.dp)
                        .background(MaterialTheme.colorScheme.primaryContainer, RoundedCornerShape(10.dp)),
                    contentAlignment = Alignment.Center,
                ) {
                    Text(
                        text = title.take(1),
                        color = MaterialTheme.colorScheme.onPrimaryContainer,
                        fontWeight = FontWeight.Black,
                    )
                }
                Text(title, fontWeight = FontWeight.Bold)
                Text(
                    subtitle,
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            if (badge) {
                WorldAlertBadge(
                    critical = true,
                    modifier = Modifier
                        .align(Alignment.TopEnd)
                        .padding(8.dp),
                )
            }
        }
    }
}

@Composable
internal fun DbaWorldMonitoringScreen(
    world: WorldState,
    russian: Boolean,
    onBack: () -> Unit,
    onOpenNode: (String) -> Unit,
) {
    val events = WorldEventEngine.eventsFor(world)

    LazyColumn(
        modifier = Modifier.fillMaxSize(),
        contentPadding = androidx.compose.foundation.layout.PaddingValues(16.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        item {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                OutlinedButton(onClick = onBack) {
                    Text(if (russian) "← Компьютер" else "← Desktop")
                }
                Text(
                    text = "MONITORING",
                    fontFamily = FontFamily.Monospace,
                    fontWeight = FontWeight.Bold,
                )
            }
        }

        if (events.isNotEmpty()) {
            item {
                Text(
                    text = if (russian) "События инфраструктуры" else "Infrastructure events",
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.Bold,
                )
            }
            events.forEach { event ->
                item(key = event.id) {
                    MonitoringEventCard(event = event, russian = russian)
                }
            }
        }

        world.nodes.forEach { node ->
            item {
                MonitoringNodeCard(
                    node = node,
                    russian = russian,
                    onClick = { onOpenNode(node.id) },
                )
            }
        }

        item {
            MonitoringInfrastructureStatusCard(
                title = "SW-01",
                subtitle = if (russian) "Сетевой коммутатор" else "Network switch",
                status = if (russian) "Норма" else "Healthy",
            )
        }
        item {
            MonitoringInfrastructureStatusCard(
                title = "UPS-01",
                subtitle = if (russian) "Резервное питание" else "Backup power",
                status = if (russian) "Норма" else "Healthy",
            )
        }
    }
}

@Composable
private fun MonitoringInfrastructureStatusCard(
    title: String,
    subtitle: String,
    status: String,
) {
    Card(modifier = Modifier.fillMaxWidth()) {
        Row(
            modifier = Modifier.padding(14.dp),
            horizontalArrangement = Arrangement.spacedBy(10.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            Box(
                modifier = Modifier
                    .size(10.dp)
                    .background(Color(0xFF3FA66A), CircleShape),
            )
            Column(modifier = Modifier.weight(1f)) {
                Text(title, fontWeight = FontWeight.Bold)
                Text(
                    subtitle,
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                )
            }
            Text(
                status,
                color = Color(0xFF3FA66A),
                style = MaterialTheme.typography.labelMedium,
            )
        }
    }
}

@Composable
private fun MonitoringAlertCard(
    title: String,
    severity: IncidentSeverity,
    russian: Boolean,
) {
    Surface(
        shape = RoundedCornerShape(14.dp),
        color = MaterialTheme.colorScheme.errorContainer,
        modifier = Modifier.fillMaxWidth(),
    ) {
        Row(
            modifier = Modifier.padding(14.dp),
            horizontalArrangement = Arrangement.spacedBy(10.dp),
            verticalAlignment = Alignment.CenterVertically,
        ) {
            WorldAlertBadge(critical = severity == IncidentSeverity.CRITICAL)
            Column {
                Text(
                    text = if (russian) "Активный алерт" else "Active alert",
                    fontWeight = FontWeight.Bold,
                )
                Text(title)
            }
        }
    }
}

@Composable
private fun MonitoringNodeCard(
    node: InfrastructureNode,
    russian: Boolean,
    onClick: () -> Unit,
) {
    val statusColor = when (node.status) {
        InfrastructureStatus.HEALTHY -> Color(0xFF3FA66A)
        InfrastructureStatus.WARNING -> Color(0xFFD3A52D)
        InfrastructureStatus.DOWN -> Color(0xFFD94A4A)
    }

    Card(
        modifier = Modifier
            .fillMaxWidth()
            .clickable(onClick = onClick),
    ) {
        Column(
            modifier = Modifier.padding(14.dp),
            verticalArrangement = Arrangement.spacedBy(10.dp),
        ) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically,
            ) {
                Row(
                    horizontalArrangement = Arrangement.spacedBy(8.dp),
                    verticalAlignment = Alignment.CenterVertically,
                ) {
                    Box(
                        modifier = Modifier
                            .size(10.dp)
                            .background(statusColor, CircleShape),
                    )
                    Text(node.id, fontWeight = FontWeight.Bold)
                }
                Text(
                    text = shellStatusLabel(node.status, russian),
                    color = statusColor,
                    style = MaterialTheme.typography.labelMedium,
                )
            }
            MonitorMetric("CPU", node.metrics.cpuPercent)
            MonitorMetric("RAM", node.metrics.ramPercent)
            MonitorMetric(if (russian) "Диск" else "Disk", node.metrics.diskPercent)
        }
    }
}

@Composable
private fun MonitorMetric(label: String, value: Int) {
    Row(
        modifier = Modifier.fillMaxWidth(),
        verticalAlignment = Alignment.CenterVertically,
        horizontalArrangement = Arrangement.spacedBy(10.dp),
    ) {
        Text(label, modifier = Modifier.width(52.dp), style = MaterialTheme.typography.labelSmall)
        LinearProgressIndicator(
            progress = { value.coerceIn(0, 100) / 100f },
            modifier = Modifier.weight(1f),
        )
        Text("$value%", modifier = Modifier.width(40.dp), fontFamily = FontFamily.Monospace)
    }
}

@Composable
internal fun WorldAlertBadge(
    critical: Boolean,
    modifier: Modifier = Modifier,
) {
    val color = if (critical) Color(0xFFD83F3F) else Color(0xFFD79A25)
    val transition = rememberInfiniteTransition(label = "world-alert")
    val pulse by transition.animateFloat(
        initialValue = 1f,
        targetValue = 1.12f,
        animationSpec = infiniteRepeatable(
            animation = tween(700, easing = LinearEasing),
            repeatMode = RepeatMode.Reverse,
        ),
        label = "world-alert-pulse",
    )
    Box(
        modifier = modifier
            .graphicsLayer(scaleX = pulse, scaleY = pulse)
            .size(30.dp)
            .background(color, CircleShape)
            .border(2.dp, Color.White.copy(alpha = 0.85f), CircleShape),
        contentAlignment = Alignment.Center,
    ) {
        Text("!", color = Color.White, fontWeight = FontWeight.Black)
    }
}

private fun outdoorSky(dayPhase: WorldDayPhase): List<Color> = when (dayPhase) {
    WorldDayPhase.MORNING -> listOf(Color(0xFF7FA9C3), Color(0xFFF2C49A))
    WorldDayPhase.DAY -> listOf(Color(0xFF67A8D5), Color(0xFFBBD8E8))
    WorldDayPhase.EVENING -> listOf(Color(0xFF3B4F78), Color(0xFFC67A62))
    WorldDayPhase.NIGHT -> listOf(Color(0xFF0B1325), Color(0xFF1C2940))
}

private fun weatherLabel(environment: WorldEnvironment, russian: Boolean): String {
    val season = when (environment.season) {
        WorldSeason.WINTER -> if (russian) "зима" else "winter"
        WorldSeason.SPRING -> if (russian) "весна" else "spring"
        WorldSeason.SUMMER -> if (russian) "лето" else "summer"
        WorldSeason.AUTUMN -> if (russian) "осень" else "autumn"
    }
    val weather = when (environment.weather) {
        WorldWeather.CLEAR -> if (russian) "ясно" else "clear"
        WorldWeather.CLOUDY -> if (russian) "облачно" else "cloudy"
        WorldWeather.RAIN -> if (russian) "дождь" else "rain"
        WorldWeather.SNOW -> if (russian) "снег" else "snow"
    }
    return "$season · $weather"
}

private fun shellClockLabel(clock: WorldClock): String {
    val date = clock.date
    return "%02d.%02d.%04d · %02d:%02d".format(
        date.dayOfMonth,
        date.month,
        date.year,
        clock.hour,
        clock.minute,
    )
}

private fun shellStatusLabel(status: InfrastructureStatus, russian: Boolean): String = when (status) {
    InfrastructureStatus.HEALTHY -> if (russian) "Норма" else "Healthy"
    InfrastructureStatus.WARNING -> if (russian) "Внимание" else "Warning"
    InfrastructureStatus.DOWN -> if (russian) "Недоступен" else "Down"
}
