use serde::Deserialize;
use vbsp_common::deserialize_bool;
use vbsp_common::{Angles, Color, LightColor, Negated, Vector};
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
#[serde(tag = "classname")]
pub enum Entity<'a> {
    #[serde(rename = "ai_ally_manager")]
    #[serde(borrow)]
    AiAllyManager(AiAllyManager<'a>),
    #[serde(rename = "ai_ally_speech_manager")]
    AiAllySpeechManager(AiAllySpeechManager),
    #[serde(rename = "ai_battle_line")]
    #[serde(borrow)]
    AiBattleLine(AiBattleLine<'a>),
    #[serde(rename = "ai_changehintgroup")]
    #[serde(borrow)]
    AiChangehintgroup(AiChangehintgroup<'a>),
    #[serde(rename = "ai_changetarget")]
    #[serde(borrow)]
    AiChangetarget(AiChangetarget<'a>),
    #[serde(rename = "ai_citizen_response_system")]
    #[serde(borrow)]
    AiCitizenResponseSystem(AiCitizenResponseSystem<'a>),
    #[serde(rename = "ai_goal_actbusy")]
    #[serde(borrow)]
    AiGoalActbusy(AiGoalActbusy<'a>),
    #[serde(rename = "ai_goal_actbusy_queue")]
    #[serde(borrow)]
    AiGoalActbusyQueue(AiGoalActbusyQueue<'a>),
    #[serde(rename = "ai_goal_assault")]
    #[serde(borrow)]
    AiGoalAssault(AiGoalAssault<'a>),
    #[serde(rename = "ai_goal_follow")]
    #[serde(borrow)]
    AiGoalFollow(AiGoalFollow<'a>),
    #[serde(rename = "ai_goal_injured_follow")]
    AiGoalInjuredFollow(AiGoalInjuredFollow),
    #[serde(rename = "ai_goal_lead")]
    #[serde(borrow)]
    AiGoalLead(AiGoalLead<'a>),
    #[serde(rename = "ai_goal_lead_weapon")]
    #[serde(borrow)]
    AiGoalLeadWeapon(AiGoalLeadWeapon<'a>),
    #[serde(rename = "ai_goal_operator")]
    #[serde(borrow)]
    AiGoalOperator(AiGoalOperator<'a>),
    #[serde(rename = "ai_goal_police")]
    #[serde(borrow)]
    AiGoalPolice(AiGoalPolice<'a>),
    #[serde(rename = "ai_goal_standoff")]
    #[serde(borrow)]
    AiGoalStandoff(AiGoalStandoff<'a>),
    #[serde(rename = "ai_hint")]
    AiHint(AiHint),
    #[serde(rename = "ai_looktarget")]
    AiLooktarget(AiLooktarget),
    #[serde(rename = "ai_network")]
    AiNetwork(AiNetwork),
    #[serde(rename = "ai_network_build_helper")]
    AiNetworkBuildHelper(AiNetworkBuildHelper),
    #[serde(rename = "ai_npc_eventresponsesystem")]
    #[serde(borrow)]
    AiNpcEventresponsesystem(AiNpcEventresponsesystem<'a>),
    #[serde(rename = "ai_relationship")]
    #[serde(borrow)]
    AiRelationship(AiRelationship<'a>),
    #[serde(rename = "ai_script_conditions")]
    #[serde(borrow)]
    AiScriptConditions(AiScriptConditions<'a>),
    #[serde(rename = "ai_sound")]
    #[serde(borrow)]
    AiSound(AiSound<'a>),
    #[serde(rename = "ai_speechfilter")]
    #[serde(borrow)]
    AiSpeechfilter(AiSpeechfilter<'a>),
    #[serde(rename = "aiscripted_schedule")]
    #[serde(borrow)]
    AiscriptedSchedule(AiscriptedSchedule<'a>),
    #[serde(rename = "aitesthull")]
    #[serde(borrow)]
    Aitesthull(Aitesthull<'a>),
    #[serde(rename = "ambient_generic")]
    #[serde(borrow)]
    AmbientGeneric(AmbientGeneric<'a>),
    #[serde(rename = "apc_missile")]
    ApcMissile(ApcMissile),
    #[serde(rename = "apc_missile")]
    ApcMissile(ApcMissile),
    #[serde(rename = "ar2explosion")]
    Ar2explosion(Ar2explosion),
    #[serde(rename = "archer_proxy")]
    #[serde(borrow)]
    ArcherProxy(ArcherProxy<'a>),
    #[serde(rename = "assault_assaultpoint")]
    #[serde(borrow)]
    AssaultAssaultpoint(AssaultAssaultpoint<'a>),
    #[serde(rename = "assault_rallypoint")]
    #[serde(borrow)]
    AssaultRallypoint(AssaultRallypoint<'a>),
    #[serde(rename = "_ballplayertoucher")]
    #[serde(borrow)]
    Ballplayertoucher(Ballplayertoucher<'a>),
    #[serde(rename = "base_boss")]
    #[serde(borrow)]
    BaseBoss(BaseBoss<'a>),
    #[serde(rename = "base_boss")]
    BaseBoss(BaseBoss),
    #[serde(rename = "base_zombie")]
    BaseZombie(BaseZombie),
    #[serde(rename = "basehl2mpcombatweapon")]
    Basehl2mpcombatweapon(Basehl2mpcombatweapon),
    #[serde(rename = "basehlcombatweapon")]
    Basehlcombatweapon(Basehlcombatweapon),
    #[serde(rename = "baseprojectile")]
    Baseprojectile(Baseprojectile),
    #[serde(rename = "beam")]
    #[serde(borrow)]
    Beam(Beam<'a>),
    #[serde(rename = "blob_element")]
    #[serde(borrow)]
    BlobElement(BlobElement<'a>),
    #[serde(rename = "bodyque")]
    #[serde(borrow)]
    Bodyque(Bodyque<'a>),
    #[serde(rename = "boss_alpha")]
    BossAlpha(BossAlpha),
    #[serde(rename = "bot_action_point")]
    #[serde(borrow)]
    BotActionPoint(BotActionPoint<'a>),
    #[serde(rename = "bot_action_point")]
    #[serde(borrow)]
    BotActionPoint(BotActionPoint<'a>),
    #[serde(rename = "bot_boss")]
    BotBoss(BotBoss),
    #[serde(rename = "bot_boss_mini_nuker")]
    BotBossMiniNuker(BotBossMiniNuker),
    #[serde(rename = "bot_boss_mini_rockets")]
    BotBossMiniRockets(BotBossMiniRockets),
    #[serde(rename = "bot_controller")]
    #[serde(borrow)]
    BotController(BotController<'a>),
    #[serde(rename = "bot_generator")]
    #[serde(borrow)]
    BotGenerator(BotGenerator<'a>),
    #[serde(rename = "bot_generator")]
    #[serde(borrow)]
    BotGenerator(BotGenerator<'a>),
    #[serde(rename = "bot_hint_engineer_nest")]
    BotHintEngineerNest(BotHintEngineerNest),
    #[serde(rename = "bot_hint_sentrygun")]
    BotHintSentrygun(BotHintSentrygun),
    #[serde(rename = "bot_hint_teleporter_exit")]
    BotHintTeleporterExit(BotHintTeleporterExit),
    #[serde(rename = "bot_npc_archer")]
    BotNpcArcher(BotNpcArcher),
    #[serde(rename = "bot_npc_decoy")]
    BotNpcDecoy(BotNpcDecoy),
    #[serde(rename = "bot_npc_minion")]
    BotNpcMinion(BotNpcMinion),
    #[serde(rename = "bot_proxy")]
    #[serde(borrow)]
    BotProxy(BotProxy<'a>),
    #[serde(rename = "bot_proxy")]
    #[serde(borrow)]
    BotProxy(BotProxy<'a>),
    #[serde(rename = "bot_roster")]
    #[serde(borrow)]
    BotRoster(BotRoster<'a>),
    #[serde(rename = "bounce_bomb")]
    #[serde(borrow)]
    BounceBomb(BounceBomb<'a>),
    #[serde(rename = "brickbat")]
    Brickbat(Brickbat),
    #[serde(rename = "bullseye_strider_focus")]
    #[serde(borrow)]
    BullseyeStriderFocus(BullseyeStriderFocus<'a>),
    #[serde(rename = "client_ragdoll")]
    ClientRagdoll(ClientRagdoll),
    #[serde(rename = "color_correction")]
    #[serde(borrow)]
    ColorCorrection(ColorCorrection<'a>),
    #[serde(rename = "color_correction_volume")]
    #[serde(borrow)]
    ColorCorrectionVolume(ColorCorrectionVolume<'a>),
    #[serde(rename = "combine_armor_piece")]
    #[serde(borrow)]
    CombineArmorPiece(CombineArmorPiece<'a>),
    #[serde(rename = "combine_bouncemine")]
    #[serde(borrow)]
    CombineBouncemine(CombineBouncemine<'a>),
    #[serde(rename = "combine_mine")]
    #[serde(borrow)]
    CombineMine(CombineMine<'a>),
    #[serde(rename = "commentary_auto")]
    #[serde(borrow)]
    CommentaryAuto(CommentaryAuto<'a>),
    #[serde(rename = "concussiveblast")]
    #[serde(borrow)]
    Concussiveblast(Concussiveblast<'a>),
    #[serde(rename = "crane_tip")]
    #[serde(borrow)]
    CraneTip(CraneTip<'a>),
    #[serde(rename = "crossbow_bolt")]
    #[serde(borrow)]
    CrossbowBolt(CrossbowBolt<'a>),
    #[serde(rename = "crossbow_bolt")]
    #[serde(borrow)]
    CrossbowBolt(CrossbowBolt<'a>),
    #[serde(rename = "cycler")]
    Cycler(Cycler),
    #[serde(rename = "cycler_actor")]
    #[serde(borrow)]
    CyclerActor(CyclerActor<'a>),
    #[serde(rename = "cycler_blender")]
    #[serde(borrow)]
    CyclerBlender(CyclerBlender<'a>),
    #[serde(rename = "cycler_flex")]
    #[serde(borrow)]
    CyclerFlex(CyclerFlex<'a>),
    #[serde(rename = "cycler_weapon")]
    CyclerWeapon(CyclerWeapon),
    #[serde(rename = "cycler_wreckage")]
    #[serde(borrow)]
    CyclerWreckage(CyclerWreckage<'a>),
    #[serde(rename = "DelayedUse")]
    DelayedUse(DelayedUse),
    #[serde(rename = "dispenser_touch_trigger")]
    #[serde(borrow)]
    DispenserTouchTrigger(DispenserTouchTrigger<'a>),
    #[serde(rename = "dynamic_prop")]
    #[serde(borrow)]
    DynamicProp(DynamicProp<'a>),
    #[serde(rename = "ent_watery_leech")]
    #[serde(borrow)]
    EntWateryLeech(EntWateryLeech<'a>),
    #[serde(rename = "entity_bird")]
    #[serde(borrow)]
    EntityBird(EntityBird<'a>),
    #[serde(rename = "entity_blocker")]
    #[serde(borrow)]
    EntityBlocker(EntityBlocker<'a>),
    #[serde(rename = "entity_carrier")]
    EntityCarrier(EntityCarrier),
    #[serde(rename = "entity_croc")]
    #[serde(borrow)]
    EntityCroc(EntityCroc<'a>),
    #[serde(rename = "entity_medigun_shield")]
    #[serde(borrow)]
    EntityMedigunShield(EntityMedigunShield<'a>),
    #[serde(rename = "entityName")]
    EntityName(EntityName),
    #[serde(rename = "entity_revive_marker")]
    #[serde(borrow)]
    EntityReviveMarker(EntityReviveMarker<'a>),
    #[serde(rename = "entity_rocket")]
    EntityRocket(EntityRocket),
    #[serde(rename = "entity_saucer")]
    EntitySaucer(EntitySaucer),
    #[serde(rename = "entity_sign")]
    EntitySign(EntitySign),
    #[serde(rename = "entity_soldier_statue")]
    #[serde(borrow)]
    EntitySoldierStatue(EntitySoldierStatue<'a>),
    #[serde(rename = "entity_spawn_manager")]
    #[serde(borrow)]
    EntitySpawnManager(EntitySpawnManager<'a>),
    #[serde(rename = "entity_spawn_point")]
    #[serde(borrow)]
    EntitySpawnPoint(EntitySpawnPoint<'a>),
    #[serde(rename = "entityflame")]
    #[serde(borrow)]
    Entityflame(Entityflame<'a>),
    #[serde(rename = "entityname")]
    Entityname(Entityname),
    #[serde(rename = "env_alyxemp")]
    #[serde(borrow)]
    EnvAlyxemp(EnvAlyxemp<'a>),
    #[serde(rename = "env_ar2explosion")]
    #[serde(borrow)]
    EnvAr2explosion(EnvAr2explosion<'a>),
    #[serde(rename = "env_assassinsmoke")]
    EnvAssassinsmoke(EnvAssassinsmoke),
    #[serde(rename = "env_beam")]
    #[serde(borrow)]
    EnvBeam(EnvBeam<'a>),
    #[serde(rename = "env_beverage")]
    #[serde(borrow)]
    EnvBeverage(EnvBeverage<'a>),
    #[serde(rename = "env_blood")]
    EnvBlood(EnvBlood),
    #[serde(rename = "env_bubbles")]
    #[serde(borrow)]
    EnvBubbles(EnvBubbles<'a>),
    #[serde(rename = "env_citadel_energy_core")]
    #[serde(borrow)]
    EnvCitadelEnergyCore(EnvCitadelEnergyCore<'a>),
    #[serde(rename = "env_credits")]
    EnvCredits(EnvCredits),
    #[serde(rename = "env_debughistory")]
    #[serde(borrow)]
    EnvDebughistory(EnvDebughistory<'a>),
    #[serde(rename = "env_detail_controller")]
    #[serde(borrow)]
    EnvDetailController(EnvDetailController<'a>),
    #[serde(rename = "env_dustpuff")]
    EnvDustpuff(EnvDustpuff),
    #[serde(rename = "env_dusttrail")]
    EnvDusttrail(EnvDusttrail),
    #[serde(rename = "env_effectscript")]
    #[serde(borrow)]
    EnvEffectscript(EnvEffectscript<'a>),
    #[serde(rename = "env_embers")]
    EnvEmbers(EnvEmbers),
    #[serde(rename = "env_entity_dissolver")]
    #[serde(borrow)]
    EnvEntityDissolver(EnvEntityDissolver<'a>),
    #[serde(rename = "env_entity_igniter")]
    #[serde(borrow)]
    EnvEntityIgniter(EnvEntityIgniter<'a>),
    #[serde(rename = "env_entity_maker")]
    #[serde(borrow)]
    EnvEntityMaker(EnvEntityMaker<'a>),
    #[serde(rename = "env_explosion")]
    #[serde(borrow)]
    EnvExplosion(EnvExplosion<'a>),
    #[serde(rename = "env_extinguisherjet")]
    #[serde(borrow)]
    EnvExtinguisherjet(EnvExtinguisherjet<'a>),
    #[serde(rename = "env_fade")]
    EnvFade(EnvFade),
    #[serde(rename = "env_fire")]
    #[serde(borrow)]
    EnvFire(EnvFire<'a>),
    #[serde(rename = "env_fire_trail")]
    EnvFireTrail(EnvFireTrail),
    #[serde(rename = "env_firesensor")]
    #[serde(borrow)]
    EnvFiresensor(EnvFiresensor<'a>),
    #[serde(rename = "env_firesource")]
    #[serde(borrow)]
    EnvFiresource(EnvFiresource<'a>),
    #[serde(rename = "env_flare")]
    #[serde(borrow)]
    EnvFlare(EnvFlare<'a>),
    #[serde(rename = "env_fog_controller")]
    #[serde(borrow)]
    EnvFogController(EnvFogController<'a>),
    #[serde(rename = "env_funnel")]
    #[serde(borrow)]
    EnvFunnel(EnvFunnel<'a>),
    #[serde(rename = "env_global")]
    #[serde(borrow)]
    EnvGlobal(EnvGlobal<'a>),
    #[serde(rename = "env_glow")]
    EnvGlow(EnvGlow),
    #[serde(rename = "env_gunfire")]
    #[serde(borrow)]
    EnvGunfire(EnvGunfire<'a>),
    #[serde(rename = "env_headcrabcanister")]
    #[serde(borrow)]
    EnvHeadcrabcanister(EnvHeadcrabcanister<'a>),
    #[serde(rename = "env_hudhint")]
    #[serde(borrow)]
    EnvHudhint(EnvHudhint<'a>),
    #[serde(rename = "env_laser")]
    #[serde(borrow)]
    EnvLaser(EnvLaser<'a>),
    #[serde(rename = "env_laserdot")]
    EnvLaserdot(EnvLaserdot),
    #[serde(rename = "env_laserdot")]
    EnvLaserdot(EnvLaserdot),
    #[serde(rename = "env_laserdot")]
    EnvLaserdot(EnvLaserdot),
    #[serde(rename = "env_lightglow")]
    #[serde(borrow)]
    EnvLightglow(EnvLightglow<'a>),
    #[serde(rename = "env_message")]
    #[serde(borrow)]
    EnvMessage(EnvMessage<'a>),
    #[serde(rename = "env_microphone")]
    #[serde(borrow)]
    EnvMicrophone(EnvMicrophone<'a>),
    #[serde(rename = "env_movieexplosion")]
    EnvMovieexplosion(EnvMovieexplosion),
    #[serde(rename = "env_muzzleflash")]
    #[serde(borrow)]
    EnvMuzzleflash(EnvMuzzleflash<'a>),
    #[serde(rename = "env_particle_performance_monitor")]
    EnvParticlePerformanceMonitor(EnvParticlePerformanceMonitor),
    #[serde(rename = "env_particle_trail")]
    EnvParticleTrail(EnvParticleTrail),
    #[serde(rename = "env_particlefire")]
    EnvParticlefire(EnvParticlefire),
    #[serde(rename = "env_particlelight")]
    #[serde(borrow)]
    EnvParticlelight(EnvParticlelight<'a>),
    #[serde(rename = "env_particlescript")]
    #[serde(borrow)]
    EnvParticlescript(EnvParticlescript<'a>),
    #[serde(rename = "env_particlesmokegrenade")]
    EnvParticlesmokegrenade(EnvParticlesmokegrenade),
    #[serde(rename = "env_physexplosion")]
    #[serde(borrow)]
    EnvPhysexplosion(EnvPhysexplosion<'a>),
    #[serde(rename = "env_physimpact")]
    #[serde(borrow)]
    EnvPhysimpact(EnvPhysimpact<'a>),
    #[serde(rename = "env_physwire")]
    EnvPhyswire(EnvPhyswire),
    #[serde(rename = "env_player_surface_trigger")]
    EnvPlayerSurfaceTrigger(EnvPlayerSurfaceTrigger),
    #[serde(rename = "env_projectedtexture")]
    EnvProjectedtexture(EnvProjectedtexture),
    #[serde(rename = "env_quadraticbeam")]
    EnvQuadraticbeam(EnvQuadraticbeam),
    #[serde(rename = "env_ragdoll_boogie")]
    #[serde(borrow)]
    EnvRagdollBoogie(EnvRagdollBoogie<'a>),
    #[serde(rename = "env_rockettrail")]
    EnvRockettrail(EnvRockettrail),
    #[serde(rename = "env_rotorshooter")]
    #[serde(borrow)]
    EnvRotorshooter(EnvRotorshooter<'a>),
    #[serde(rename = "env_rotorwash_emitter")]
    #[serde(borrow)]
    EnvRotorwashEmitter(EnvRotorwashEmitter<'a>),
    #[serde(rename = "env_screeneffect")]
    EnvScreeneffect(EnvScreeneffect),
    #[serde(rename = "env_screenoverlay")]
    #[serde(borrow)]
    EnvScreenoverlay(EnvScreenoverlay<'a>),
    #[serde(rename = "env_shake")]
    EnvShake(EnvShake),
    #[serde(rename = "env_shooter")]
    #[serde(borrow)]
    EnvShooter(EnvShooter<'a>),
    #[serde(rename = "env_smokestack")]
    EnvSmokestack(EnvSmokestack),
    #[serde(rename = "env_smoketrail")]
    EnvSmoketrail(EnvSmoketrail),
    #[serde(rename = "env_sniperdot")]
    #[serde(borrow)]
    EnvSniperdot(EnvSniperdot<'a>),
    #[serde(rename = "env_soundscape")]
    #[serde(borrow)]
    EnvSoundscape(EnvSoundscape<'a>),
    #[serde(rename = "env_soundscape_proxy")]
    #[serde(borrow)]
    EnvSoundscapeProxy(EnvSoundscapeProxy<'a>),
    #[serde(rename = "env_soundscape_triggerable")]
    #[serde(borrow)]
    EnvSoundscapeTriggerable(EnvSoundscapeTriggerable<'a>),
    #[serde(rename = "env_spark")]
    EnvSpark(EnvSpark),
    #[serde(rename = "env_sparkler")]
    #[serde(borrow)]
    EnvSparkler(EnvSparkler<'a>),
    #[serde(rename = "env_speaker")]
    #[serde(borrow)]
    EnvSpeaker(EnvSpeaker<'a>),
    #[serde(rename = "env_splash")]
    EnvSplash(EnvSplash),
    #[serde(rename = "env_sporeexplosion")]
    EnvSporeexplosion(EnvSporeexplosion),
    #[serde(rename = "env_sporetrail")]
    EnvSporetrail(EnvSporetrail),
    #[serde(rename = "env_sprite")]
    EnvSprite(EnvSprite),
    #[serde(rename = "env_sprite_oriented")]
    EnvSpriteOriented(EnvSpriteOriented),
    #[serde(rename = "env_spritetrail")]
    #[serde(borrow)]
    EnvSpritetrail(EnvSpritetrail<'a>),
    #[serde(rename = "env_starfield")]
    #[serde(borrow)]
    EnvStarfield(EnvStarfield<'a>),
    #[serde(rename = "env_steam")]
    EnvSteam(EnvSteam),
    #[serde(rename = "env_steamjet")]
    EnvSteamjet(EnvSteamjet),
    #[serde(rename = "env_sun")]
    #[serde(borrow)]
    EnvSun(EnvSun<'a>),
    #[serde(rename = "env_texturetoggle")]
    EnvTexturetoggle(EnvTexturetoggle),
    #[serde(rename = "env_tonemap_controller")]
    EnvTonemapController(EnvTonemapController),
    #[serde(rename = "env_tracer")]
    EnvTracer(EnvTracer),
    #[serde(rename = "env_viewpunch")]
    EnvViewpunch(EnvViewpunch),
    #[serde(rename = "env_wind")]
    #[serde(borrow)]
    EnvWind(EnvWind<'a>),
    #[serde(rename = "env_zoom")]
    EnvZoom(EnvZoom),
    #[serde(rename = "event_queue_saveload_proxy")]
    EventQueueSaveloadProxy(EventQueueSaveloadProxy),
    #[serde(rename = "eyeball_boss")]
    EyeballBoss(EyeballBoss),
    #[serde(rename = "filter_activator_class")]
    #[serde(borrow)]
    FilterActivatorClass(FilterActivatorClass<'a>),
    #[serde(rename = "filter_activator_mass_greater")]
    FilterActivatorMassGreater(FilterActivatorMassGreater),
    #[serde(rename = "filter_activator_name")]
    #[serde(borrow)]
    FilterActivatorName(FilterActivatorName<'a>),
    #[serde(rename = "filter_activator_team")]
    FilterActivatorTeam(FilterActivatorTeam),
    #[serde(rename = "filter_activator_tfteam")]
    #[serde(borrow)]
    FilterActivatorTfteam(FilterActivatorTfteam<'a>),
    #[serde(rename = "filter_base")]
    FilterBase(FilterBase),
    #[serde(rename = "filter_combineball_type")]
    FilterCombineballType(FilterCombineballType),
    #[serde(rename = "filter_damage_type")]
    FilterDamageType(FilterDamageType),
    #[serde(rename = "filter_enemy")]
    #[serde(borrow)]
    FilterEnemy(FilterEnemy<'a>),
    #[serde(rename = "filter_multi")]
    #[serde(borrow)]
    FilterMulti(FilterMulti<'a>),
    #[serde(rename = "filter_tf_bot_has_tag")]
    #[serde(borrow)]
    FilterTfBotHasTag(FilterTfBotHasTag<'a>),
    #[serde(rename = "filter_tf_class")]
    FilterTfClass(FilterTfClass),
    #[serde(rename = "filter_tf_condition")]
    FilterTfCondition(FilterTfCondition),
    #[serde(rename = "filter_tf_damaged_by_weapon_in_slot")]
    FilterTfDamagedByWeaponInSlot(FilterTfDamagedByWeaponInSlot),
    #[serde(rename = "filter_tf_player_can_cap")]
    FilterTfPlayerCanCap(FilterTfPlayerCanCap),
    #[serde(rename = "_firesmoke")]
    Firesmoke(Firesmoke),
    #[serde(rename = "fish")]
    #[serde(borrow)]
    Fish(Fish<'a>),
    #[serde(rename = "floorturret_tipcontroller")]
    FloorturretTipcontroller(FloorturretTipcontroller),
    #[serde(rename = "funCBaseFlex")]
    FunCBaseFlex(FunCBaseFlex),
    #[serde(rename = "func_achievement")]
    #[serde(borrow)]
    FuncAchievement(FuncAchievement<'a>),
    #[serde(rename = "func_areaportal")]
    FuncAreaportal(FuncAreaportal),
    #[serde(rename = "func_areaportalwindow")]
    #[serde(borrow)]
    FuncAreaportalwindow(FuncAreaportalwindow<'a>),
    #[serde(rename = "func_breakable")]
    #[serde(borrow)]
    FuncBreakable(FuncBreakable<'a>),
    #[serde(rename = "func_breakable_surf")]
    #[serde(borrow)]
    FuncBreakableSurf(FuncBreakableSurf<'a>),
    #[serde(rename = "func_brush")]
    #[serde(borrow)]
    FuncBrush(FuncBrush<'a>),
    #[serde(rename = "func_bulletshield")]
    #[serde(borrow)]
    FuncBulletshield(FuncBulletshield<'a>),
    #[serde(rename = "func_button")]
    #[serde(borrow)]
    FuncButton(FuncButton<'a>),
    #[serde(rename = "func_capturezone")]
    FuncCapturezone(FuncCapturezone),
    #[serde(rename = "func_changeclass")]
    #[serde(borrow)]
    FuncChangeclass(FuncChangeclass<'a>),
    #[serde(rename = "func_clip_vphysics")]
    #[serde(borrow)]
    FuncClipVphysics(FuncClipVphysics<'a>),
    #[serde(rename = "func_combine_ball_spawner")]
    #[serde(borrow)]
    FuncCombineBallSpawner(FuncCombineBallSpawner<'a>),
    #[serde(rename = "func_conveyor")]
    FuncConveyor(FuncConveyor),
    #[serde(rename = "func_croc")]
    #[serde(borrow)]
    FuncCroc(FuncCroc<'a>),
    #[serde(rename = "func_door")]
    #[serde(borrow)]
    FuncDoor(FuncDoor<'a>),
    #[serde(rename = "func_door_rotating")]
    #[serde(borrow)]
    FuncDoorRotating(FuncDoorRotating<'a>),
    #[serde(rename = "func_dustcloud")]
    FuncDustcloud(FuncDustcloud),
    #[serde(rename = "func_dustmotes")]
    FuncDustmotes(FuncDustmotes),
    #[serde(rename = "func_extinguishercharger")]
    #[serde(borrow)]
    FuncExtinguishercharger(FuncExtinguishercharger<'a>),
    #[serde(rename = "func_fish_pool")]
    #[serde(borrow)]
    FuncFishPool(FuncFishPool<'a>),
    #[serde(rename = "func_flag_alert")]
    #[serde(borrow)]
    FuncFlagAlert(FuncFlagAlert<'a>),
    #[serde(rename = "func_flagdetectionzone")]
    #[serde(borrow)]
    FuncFlagdetectionzone(FuncFlagdetectionzone<'a>),
    #[serde(rename = "func_forcefield")]
    #[serde(borrow)]
    FuncForcefield(FuncForcefield<'a>),
    #[serde(rename = "func_friction")]
    #[serde(borrow)]
    FuncFriction(FuncFriction<'a>),
    #[serde(rename = "func_guntarget")]
    #[serde(borrow)]
    FuncGuntarget(FuncGuntarget<'a>),
    #[serde(rename = "func_healthcharger")]
    #[serde(borrow)]
    FuncHealthcharger(FuncHealthcharger<'a>),
    #[serde(rename = "func_illusionary")]
    #[serde(borrow)]
    FuncIllusionary(FuncIllusionary<'a>),
    #[serde(rename = "func_ladderendpoint")]
    #[serde(borrow)]
    FuncLadderendpoint(FuncLadderendpoint<'a>),
    #[serde(rename = "func_lod")]
    #[serde(borrow)]
    FuncLod(FuncLod<'a>),
    #[serde(rename = "func_lookdoor")]
    FuncLookdoor(FuncLookdoor),
    #[serde(rename = "func_monitor")]
    #[serde(borrow)]
    FuncMonitor(FuncMonitor<'a>),
    #[serde(rename = "func_movelinear")]
    #[serde(borrow)]
    FuncMovelinear(FuncMovelinear<'a>),
    #[serde(rename = "func_nav_avoid")]
    #[serde(borrow)]
    FuncNavAvoid(FuncNavAvoid<'a>),
    #[serde(rename = "func_nav_avoidance_obstacle")]
    #[serde(borrow)]
    FuncNavAvoidanceObstacle(FuncNavAvoidanceObstacle<'a>),
    #[serde(rename = "func_nav_blocker")]
    #[serde(borrow)]
    FuncNavBlocker(FuncNavBlocker<'a>),
    #[serde(rename = "func_nav_prefer")]
    #[serde(borrow)]
    FuncNavPrefer(FuncNavPrefer<'a>),
    #[serde(rename = "func_nav_prerequisite")]
    #[serde(borrow)]
    FuncNavPrerequisite(FuncNavPrerequisite<'a>),
    #[serde(rename = "func_nobuild")]
    #[serde(borrow)]
    FuncNobuild(FuncNobuild<'a>),
    #[serde(rename = "func_nogrenades")]
    #[serde(borrow)]
    FuncNogrenades(FuncNogrenades<'a>),
    #[serde(rename = "func_occluder")]
    #[serde(borrow)]
    FuncOccluder(FuncOccluder<'a>),
    #[serde(rename = "func_passtime_goal")]
    FuncPasstimeGoal(FuncPasstimeGoal),
    #[serde(rename = "func_passtime_goalie_zone")]
    #[serde(borrow)]
    FuncPasstimeGoalieZone(FuncPasstimeGoalieZone<'a>),
    #[serde(rename = "func_passtime_no_ball_zone")]
    #[serde(borrow)]
    FuncPasstimeNoBallZone(FuncPasstimeNoBallZone<'a>),
    #[serde(rename = "func_physbox")]
    #[serde(borrow)]
    FuncPhysbox(FuncPhysbox<'a>),
    #[serde(rename = "func_physbox_multiplayer")]
    #[serde(borrow)]
    FuncPhysboxMultiplayer(FuncPhysboxMultiplayer<'a>),
    #[serde(rename = "func_plat")]
    FuncPlat(FuncPlat),
    #[serde(rename = "func_platrot")]
    FuncPlatrot(FuncPlatrot),
    #[serde(rename = "func_powerupvolume")]
    FuncPowerupvolume(FuncPowerupvolume),
    #[serde(rename = "func_precipitation")]
    #[serde(borrow)]
    FuncPrecipitation(FuncPrecipitation<'a>),
    #[serde(rename = "func_proprrespawnzone")]
    #[serde(borrow)]
    FuncProprrespawnzone(FuncProprrespawnzone<'a>),
    #[serde(rename = "func_pushable")]
    #[serde(borrow)]
    FuncPushable(FuncPushable<'a>),
    #[serde(rename = "func_recharge")]
    #[serde(borrow)]
    FuncRecharge(FuncRecharge<'a>),
    #[serde(rename = "func_reflective_glass")]
    #[serde(borrow)]
    FuncReflectiveGlass(FuncReflectiveGlass<'a>),
    #[serde(rename = "func_regenerate")]
    #[serde(borrow)]
    FuncRegenerate(FuncRegenerate<'a>),
    #[serde(rename = "func_respawnflag")]
    #[serde(borrow)]
    FuncRespawnflag(FuncRespawnflag<'a>),
    #[serde(rename = "func_respawnroom")]
    FuncRespawnroom(FuncRespawnroom),
    #[serde(rename = "func_respawnroomvisualizer")]
    #[serde(borrow)]
    FuncRespawnroomvisualizer(FuncRespawnroomvisualizer<'a>),
    #[serde(rename = "func_rot_button")]
    FuncRotButton(FuncRotButton),
    #[serde(rename = "func_rotating")]
    #[serde(borrow)]
    FuncRotating(FuncRotating<'a>),
    #[serde(rename = "func_smokevolume")]
    #[serde(borrow)]
    FuncSmokevolume(FuncSmokevolume<'a>),
    #[serde(rename = "func_suggested_build")]
    #[serde(borrow)]
    FuncSuggestedBuild(FuncSuggestedBuild<'a>),
    #[serde(rename = "func_tank")]
    #[serde(borrow)]
    FuncTank(FuncTank<'a>),
    #[serde(rename = "func_tank_combine_cannon")]
    FuncTankCombineCannon(FuncTankCombineCannon),
    #[serde(rename = "func_tankairboatgun")]
    #[serde(borrow)]
    FuncTankairboatgun(FuncTankairboatgun<'a>),
    #[serde(rename = "func_tankapcrocket")]
    #[serde(borrow)]
    FuncTankapcrocket(FuncTankapcrocket<'a>),
    #[serde(rename = "func_tanklaser")]
    #[serde(borrow)]
    FuncTanklaser(FuncTanklaser<'a>),
    #[serde(rename = "func_tankmortar")]
    #[serde(borrow)]
    FuncTankmortar(FuncTankmortar<'a>),
    #[serde(rename = "func_tankphyscannister")]
    #[serde(borrow)]
    FuncTankphyscannister(FuncTankphyscannister<'a>),
    #[serde(rename = "func_tankpulselaser")]
    FuncTankpulselaser(FuncTankpulselaser),
    #[serde(rename = "func_tankrocket")]
    #[serde(borrow)]
    FuncTankrocket(FuncTankrocket<'a>),
    #[serde(rename = "func_tanktrain")]
    FuncTanktrain(FuncTanktrain),
    #[serde(rename = "func_tfbot_hint")]
    #[serde(borrow)]
    FuncTfbotHint(FuncTfbotHint<'a>),
    #[serde(rename = "func_trackautochange")]
    FuncTrackautochange(FuncTrackautochange),
    #[serde(rename = "func_trackchange")]
    FuncTrackchange(FuncTrackchange),
    #[serde(rename = "func_tracktrain")]
    #[serde(borrow)]
    FuncTracktrain(FuncTracktrain<'a>),
    #[serde(rename = "func_train")]
    FuncTrain(FuncTrain),
    #[serde(rename = "func_traincontrols")]
    #[serde(borrow)]
    FuncTraincontrols(FuncTraincontrols<'a>),
    #[serde(rename = "func_upgradestation")]
    #[serde(borrow)]
    FuncUpgradestation(FuncUpgradestation<'a>),
    #[serde(rename = "func_useableladder")]
    #[serde(borrow)]
    FuncUseableladder(FuncUseableladder<'a>),
    #[serde(rename = "func_vehicleclip")]
    #[serde(borrow)]
    FuncVehicleclip(FuncVehicleclip<'a>),
    #[serde(rename = "func_wall")]
    #[serde(borrow)]
    FuncWall(FuncWall<'a>),
    #[serde(rename = "func_wall_toggle")]
    FuncWallToggle(FuncWallToggle),
    #[serde(rename = "func_water")]
    #[serde(borrow)]
    FuncWater(FuncWater<'a>),
    #[serde(rename = "func_water_analog")]
    #[serde(borrow)]
    FuncWaterAnalog(FuncWaterAnalog<'a>),
    #[serde(rename = "func_weight_button")]
    #[serde(borrow)]
    FuncWeightButton(FuncWeightButton<'a>),
    #[serde(rename = "game_end")]
    GameEnd(GameEnd),
    #[serde(rename = "game_forcerespawn")]
    GameForcerespawn(GameForcerespawn),
    #[serde(rename = "game_gib_manager")]
    #[serde(borrow)]
    GameGibManager(GameGibManager<'a>),
    #[serde(rename = "game_intro_viewpoint")]
    #[serde(borrow)]
    GameIntroViewpoint(GameIntroViewpoint<'a>),
    #[serde(rename = "game_player_equip")]
    GamePlayerEquip(GamePlayerEquip),
    #[serde(rename = "game_player_hurt")]
    GamePlayerHurt(GamePlayerHurt),
    #[serde(rename = "game_player_team")]
    GamePlayerTeam(GamePlayerTeam),
    #[serde(rename = "game_ragdoll_manager")]
    #[serde(borrow)]
    GameRagdollManager(GameRagdollManager<'a>),
    #[serde(rename = "game_round_win")]
    GameRoundWin(GameRoundWin),
    #[serde(rename = "game_score")]
    GameScore(GameScore),
    #[serde(rename = "game_team_set")]
    GameTeamSet(GameTeamSet),
    #[serde(rename = "game_text")]
    #[serde(borrow)]
    GameText(GameText<'a>),
    #[serde(rename = "game_text_tf")]
    #[serde(borrow)]
    GameTextTf(GameTextTf<'a>),
    #[serde(rename = "game_ui")]
    #[serde(borrow)]
    GameUi(GameUi<'a>),
    #[serde(rename = "game_weapon_manager")]
    #[serde(borrow)]
    GameWeaponManager(GameWeaponManager<'a>),
    #[serde(rename = "game_zone_player")]
    GameZonePlayer(GameZonePlayer),
    #[serde(rename = "generic_actor")]
    #[serde(borrow)]
    GenericActor(GenericActor<'a>),
    #[serde(rename = "ghost")]
    Ghost(Ghost),
    #[serde(rename = "gib")]
    #[serde(borrow)]
    Gib(Gib<'a>),
    #[serde(rename = "gibshooter")]
    #[serde(borrow)]
    Gibshooter(Gibshooter<'a>),
    #[serde(rename = "gravity_pellet")]
    #[serde(borrow)]
    GravityPellet(GravityPellet<'a>),
    #[serde(rename = "grenade")]
    Grenade(Grenade),
    #[serde(rename = "grenade_ar2")]
    GrenadeAr2(GrenadeAr2),
    #[serde(rename = "grenade_beam")]
    GrenadeBeam(GrenadeBeam),
    #[serde(rename = "grenade_beam_chaser")]
    #[serde(borrow)]
    GrenadeBeamChaser(GrenadeBeamChaser<'a>),
    #[serde(rename = "grenade_beerbottle")]
    GrenadeBeerbottle(GrenadeBeerbottle),
    #[serde(rename = "grenade_energy")]
    GrenadeEnergy(GrenadeEnergy),
    #[serde(rename = "grenade_helicopter")]
    GrenadeHelicopter(GrenadeHelicopter),
    #[serde(rename = "grenade_homer")]
    GrenadeHomer(GrenadeHomer),
    #[serde(rename = "grenade_molotov")]
    GrenadeMolotov(GrenadeMolotov),
    #[serde(rename = "grenade_pathfollower")]
    GrenadePathfollower(GrenadePathfollower),
    #[serde(rename = "grenade_projectile")]
    GrenadeProjectile(GrenadeProjectile),
    #[serde(rename = "grenade_rockbb")]
    GrenadeRockbb(GrenadeRockbb),
    #[serde(rename = "grenade_spit")]
    GrenadeSpit(GrenadeSpit),
    #[serde(rename = "halloween_fortune_teller")]
    #[serde(borrow)]
    HalloweenFortuneTeller(HalloweenFortuneTeller<'a>),
    #[serde(rename = "halloween_souls_pack")]
    #[serde(borrow)]
    HalloweenSoulsPack(HalloweenSoulsPack<'a>),
    #[serde(rename = "halloween_zapper")]
    #[serde(borrow)]
    HalloweenZapper(HalloweenZapper<'a>),
    #[serde(rename = "hammer_updateignorelist")]
    #[serde(borrow)]
    HammerUpdateignorelist(HammerUpdateignorelist<'a>),
    #[serde(rename = "handle_dummy")]
    #[serde(borrow)]
    HandleDummy(HandleDummy<'a>),
    #[serde(rename = "handle_test")]
    #[serde(borrow)]
    HandleTest(HandleTest<'a>),
    #[serde(rename = "headless_hatman")]
    HeadlessHatman(HeadlessHatman),
    #[serde(rename = "helicopter_chunk")]
    #[serde(borrow)]
    HelicopterChunk(HelicopterChunk<'a>),
    #[serde(rename = "hightower_teleport_vortex")]
    #[serde(borrow)]
    HightowerTeleportVortex(HightowerTeleportVortex<'a>),
    #[serde(rename = "hl2_gamerules")]
    Hl2Gamerules(Hl2Gamerules),
    #[serde(rename = "hl2_survival_gamerules")]
    Hl2SurvivalGamerules(Hl2SurvivalGamerules),
    #[serde(rename = "hl2mp_bot")]
    Hl2mpBot(Hl2mpBot),
    #[serde(rename = "hl2mp_gamerules")]
    Hl2mpGamerules(Hl2mpGamerules),
    #[serde(rename = "hl2mp_ragdoll")]
    Hl2mpRagdoll(Hl2mpRagdoll),
    #[serde(rename = "hunter_flechette")]
    #[serde(borrow)]
    HunterFlechette(HunterFlechette<'a>),
    #[serde(rename = "hydra_impale")]
    #[serde(borrow)]
    HydraImpale(HydraImpale<'a>),
    #[serde(rename = "info_apc_missile_hint")]
    #[serde(borrow)]
    InfoApcMissileHint(InfoApcMissileHint<'a>),
    #[serde(rename = "info_apc_missile_hint")]
    #[serde(borrow)]
    InfoApcMissileHint(InfoApcMissileHint<'a>),
    #[serde(rename = "info_camera_link")]
    #[serde(borrow)]
    InfoCameraLink(InfoCameraLink<'a>),
    #[serde(rename = "info_constraint_anchor")]
    InfoConstraintAnchor(InfoConstraintAnchor),
    #[serde(rename = "info_darknessmode_lightsource")]
    #[serde(borrow)]
    InfoDarknessmodeLightsource(InfoDarknessmodeLightsource<'a>),
    #[serde(rename = "info_hint")]
    InfoHint(InfoHint),
    #[serde(rename = "info_intermission")]
    InfoIntermission(InfoIntermission),
    #[serde(rename = "info_ladder_dismount")]
    #[serde(borrow)]
    InfoLadderDismount(InfoLadderDismount<'a>),
    #[serde(rename = "info_landmark")]
    #[serde(borrow)]
    InfoLandmark(InfoLandmark<'a>),
    #[serde(rename = "info_lighting_relative")]
    #[serde(borrow)]
    InfoLightingRelative(InfoLightingRelative<'a>),
    #[serde(rename = "info_mass_center")]
    InfoMassCenter(InfoMassCenter),
    #[serde(rename = "info_node")]
    InfoNode(InfoNode),
    #[serde(rename = "info_node_air")]
    InfoNodeAir(InfoNodeAir),
    #[serde(rename = "info_node_air_hint")]
    InfoNodeAirHint(InfoNodeAirHint),
    #[serde(rename = "info_node_climb")]
    InfoNodeClimb(InfoNodeClimb),
    #[serde(rename = "info_node_hint")]
    InfoNodeHint(InfoNodeHint),
    #[serde(rename = "info_node_link")]
    #[serde(borrow)]
    InfoNodeLink(InfoNodeLink<'a>),
    #[serde(rename = "info_node_link_controller")]
    #[serde(borrow)]
    InfoNodeLinkController(InfoNodeLinkController<'a>),
    #[serde(rename = "info_npc_spawn_destination")]
    #[serde(borrow)]
    InfoNpcSpawnDestination(InfoNpcSpawnDestination<'a>),
    #[serde(rename = "info_null")]
    #[serde(borrow)]
    InfoNull(InfoNull<'a>),
    #[serde(rename = "info_observer_point")]
    #[serde(borrow)]
    InfoObserverPoint(InfoObserverPoint<'a>),
    #[serde(rename = "info_overlay_accessor")]
    InfoOverlayAccessor(InfoOverlayAccessor),
    #[serde(rename = "info_particle_system")]
    #[serde(borrow)]
    InfoParticleSystem(InfoParticleSystem<'a>),
    #[serde(rename = "info_passtime_ball_spawn")]
    InfoPasstimeBallSpawn(InfoPasstimeBallSpawn),
    #[serde(rename = "info_player_combine")]
    #[serde(borrow)]
    InfoPlayerCombine(InfoPlayerCombine<'a>),
    #[serde(rename = "info_player_counterterrorist")]
    #[serde(borrow)]
    InfoPlayerCounterterrorist(InfoPlayerCounterterrorist<'a>),
    #[serde(rename = "info_player_deathmatch")]
    #[serde(borrow)]
    InfoPlayerDeathmatch(InfoPlayerDeathmatch<'a>),
    #[serde(rename = "info_player_rebel")]
    #[serde(borrow)]
    InfoPlayerRebel(InfoPlayerRebel<'a>),
    #[serde(rename = "info_player_start")]
    #[serde(borrow)]
    InfoPlayerStart(InfoPlayerStart<'a>),
    #[serde(rename = "info_player_teamspawn")]
    InfoPlayerTeamspawn(InfoPlayerTeamspawn),
    #[serde(rename = "info_player_teamspawn")]
    #[serde(borrow)]
    InfoPlayerTeamspawn(InfoPlayerTeamspawn<'a>),
    #[serde(rename = "info_player_teamspawn_old")]
    InfoPlayerTeamspawnOld(InfoPlayerTeamspawnOld),
    #[serde(rename = "info_player_terrorist")]
    #[serde(borrow)]
    InfoPlayerTerrorist(InfoPlayerTerrorist<'a>),
    #[serde(rename = "info_populator")]
    InfoPopulator(InfoPopulator),
    #[serde(rename = "info_powerup_spawn")]
    InfoPowerupSpawn(InfoPowerupSpawn),
    #[serde(rename = "info_projecteddecal")]
    InfoProjecteddecal(InfoProjecteddecal),
    #[serde(rename = "info_radar_target")]
    InfoRadarTarget(InfoRadarTarget),
    #[serde(rename = "info_radial_link_controller")]
    #[serde(borrow)]
    InfoRadialLinkController(InfoRadialLinkController<'a>),
    #[serde(rename = "info_snipertarget")]
    #[serde(borrow)]
    InfoSnipertarget(InfoSnipertarget<'a>),
    #[serde(rename = "info_target")]
    InfoTarget(InfoTarget),
    #[serde(rename = "info_target_command_point")]
    InfoTargetCommandPoint(InfoTargetCommandPoint),
    #[serde(rename = "info_target_gunshipcrash")]
    InfoTargetGunshipcrash(InfoTargetGunshipcrash),
    #[serde(rename = "info_target_helicopter_crash")]
    #[serde(borrow)]
    InfoTargetHelicopterCrash(InfoTargetHelicopterCrash<'a>),
    #[serde(rename = "info_target_immolator")]
    #[serde(borrow)]
    InfoTargetImmolator(InfoTargetImmolator<'a>),
    #[serde(rename = "info_target_vehicle_transition")]
    InfoTargetVehicleTransition(InfoTargetVehicleTransition),
    #[serde(rename = "info_teleport_destination")]
    #[serde(borrow)]
    InfoTeleportDestination(InfoTeleportDestination<'a>),
    #[serde(rename = "info_teleporter_countdown")]
    InfoTeleporterCountdown(InfoTeleporterCountdown),
    #[serde(rename = "info_tf_teamcheck")]
    InfoTfTeamcheck(InfoTfTeamcheck),
    #[serde(rename = "info_tfgoal")]
    InfoTfgoal(InfoTfgoal),
    #[serde(rename = "info_vehicle_groundspawn")]
    InfoVehicleGroundspawn(InfoVehicleGroundspawn),
    #[serde(rename = "infodecal")]
    Infodecal(Infodecal),
    #[serde(rename = "instanced_scripted_scene")]
    #[serde(borrow)]
    InstancedScriptedScene(InstancedScriptedScene<'a>),
    #[serde(rename = "item_ammo_357")]
    ItemAmmo357(ItemAmmo357),
    #[serde(rename = "item_ammo_357_large")]
    ItemAmmo357Large(ItemAmmo357Large),
    #[serde(rename = "item_ammo_ar2")]
    ItemAmmoAr2(ItemAmmoAr2),
    #[serde(rename = "item_ammo_ar2_altfire")]
    ItemAmmoAr2Altfire(ItemAmmoAr2Altfire),
    #[serde(rename = "item_ammo_ar2_large")]
    ItemAmmoAr2Large(ItemAmmoAr2Large),
    #[serde(rename = "item_ammo_crate")]
    #[serde(borrow)]
    ItemAmmoCrate(ItemAmmoCrate<'a>),
    #[serde(rename = "item_ammo_crossbow")]
    ItemAmmoCrossbow(ItemAmmoCrossbow),
    #[serde(rename = "item_ammo_pistol")]
    ItemAmmoPistol(ItemAmmoPistol),
    #[serde(rename = "item_ammo_pistol_large")]
    ItemAmmoPistolLarge(ItemAmmoPistolLarge),
    #[serde(rename = "item_ammo_smg1")]
    ItemAmmoSmg1(ItemAmmoSmg1),
    #[serde(rename = "item_ammo_smg1_grenade")]
    ItemAmmoSmg1Grenade(ItemAmmoSmg1Grenade),
    #[serde(rename = "item_ammo_smg1_large")]
    ItemAmmoSmg1Large(ItemAmmoSmg1Large),
    #[serde(rename = "item_ammopack_full")]
    #[serde(borrow)]
    ItemAmmopackFull(ItemAmmopackFull<'a>),
    #[serde(rename = "item_ammopack_medium")]
    ItemAmmopackMedium(ItemAmmopackMedium),
    #[serde(rename = "item_ammopack_small")]
    ItemAmmopackSmall(ItemAmmopackSmall),
    #[serde(rename = "item_antidote")]
    ItemAntidote(ItemAntidote),
    #[serde(rename = "item_ar2_grenade")]
    ItemAr2Grenade(ItemAr2Grenade),
    #[serde(rename = "item_armor")]
    #[serde(borrow)]
    ItemArmor(ItemArmor<'a>),
    #[serde(rename = "item_battery")]
    ItemBattery(ItemBattery),
    #[serde(rename = "item_bonuspack")]
    ItemBonuspack(ItemBonuspack),
    #[serde(rename = "item_box_buckshot")]
    ItemBoxBuckshot(ItemBoxBuckshot),
    #[serde(rename = "item_box_flare_rounds")]
    ItemBoxFlareRounds(ItemBoxFlareRounds),
    #[serde(rename = "item_box_lrounds")]
    ItemBoxLrounds(ItemBoxLrounds),
    #[serde(rename = "item_box_mrounds")]
    ItemBoxMrounds(ItemBoxMrounds),
    #[serde(rename = "item_box_sniper_rounds")]
    ItemBoxSniperRounds(ItemBoxSniperRounds),
    #[serde(rename = "item_box_srounds")]
    ItemBoxSrounds(ItemBoxSrounds),
    #[serde(rename = "item_currencypack_custom")]
    ItemCurrencypackCustom(ItemCurrencypackCustom),
    #[serde(rename = "item_currencypack_large")]
    #[serde(borrow)]
    ItemCurrencypackLarge(ItemCurrencypackLarge<'a>),
    #[serde(rename = "item_currencypack_medium")]
    ItemCurrencypackMedium(ItemCurrencypackMedium),
    #[serde(rename = "item_currencypack_small")]
    ItemCurrencypackSmall(ItemCurrencypackSmall),
    #[serde(rename = "item_dynamic_resupply")]
    ItemDynamicResupply(ItemDynamicResupply),
    #[serde(rename = "item_flare_round")]
    ItemFlareRound(ItemFlareRound),
    #[serde(rename = "item_grenadepack")]
    #[serde(borrow)]
    ItemGrenadepack(ItemGrenadepack<'a>),
    #[serde(rename = "item_grubnugget")]
    ItemGrubnugget(ItemGrubnugget),
    #[serde(rename = "item_healthammokit")]
    ItemHealthammokit(ItemHealthammokit),
    #[serde(rename = "item_healthcharger")]
    #[serde(borrow)]
    ItemHealthcharger(ItemHealthcharger<'a>),
    #[serde(rename = "item_healthkit")]
    ItemHealthkit(ItemHealthkit),
    #[serde(rename = "item_healthkit_full")]
    ItemHealthkitFull(ItemHealthkitFull),
    #[serde(rename = "item_healthkit_medium")]
    ItemHealthkitMedium(ItemHealthkitMedium),
    #[serde(rename = "item_healthkit_small")]
    ItemHealthkitSmall(ItemHealthkitSmall),
    #[serde(rename = "item_healthvial")]
    ItemHealthvial(ItemHealthvial),
    #[serde(rename = "item_item_crate")]
    #[serde(borrow)]
    ItemItemCrate(ItemItemCrate<'a>),
    #[serde(rename = "item_large_box_lrounds")]
    ItemLargeBoxLrounds(ItemLargeBoxLrounds),
    #[serde(rename = "item_large_box_mrounds")]
    ItemLargeBoxMrounds(ItemLargeBoxMrounds),
    #[serde(rename = "item_large_box_srounds")]
    ItemLargeBoxSrounds(ItemLargeBoxSrounds),
    #[serde(rename = "item_longjump")]
    ItemLongjump(ItemLongjump),
    #[serde(rename = "item_ml_grenade")]
    ItemMlGrenade(ItemMlGrenade),
    #[serde(rename = "item_powerup_crit")]
    ItemPowerupCrit(ItemPowerupCrit),
    #[serde(rename = "item_powerup_rune")]
    #[serde(borrow)]
    ItemPowerupRune(ItemPowerupRune<'a>),
    #[serde(rename = "item_powerup_rune_temp")]
    #[serde(borrow)]
    ItemPowerupRuneTemp(ItemPowerupRuneTemp<'a>),
    #[serde(rename = "item_powerup_uber")]
    ItemPowerupUber(ItemPowerupUber),
    #[serde(rename = "item_rpg_round")]
    ItemRpgRound(ItemRpgRound),
    #[serde(rename = "item_security")]
    ItemSecurity(ItemSecurity),
    #[serde(rename = "item_sodacan")]
    #[serde(borrow)]
    ItemSodacan(ItemSodacan<'a>),
    #[serde(rename = "item_suit")]
    ItemSuit(ItemSuit),
    #[serde(rename = "item_suitcharger")]
    #[serde(borrow)]
    ItemSuitcharger(ItemSuitcharger<'a>),
    #[serde(rename = "item_teamflag")]
    #[serde(borrow)]
    ItemTeamflag(ItemTeamflag<'a>),
    #[serde(rename = "item_teamflag_return_icon")]
    #[serde(borrow)]
    ItemTeamflagReturnIcon(ItemTeamflagReturnIcon<'a>),
    #[serde(rename = "item_tfgoal")]
    ItemTfgoal(ItemTfgoal),
    #[serde(rename = "keyframe_rope")]
    #[serde(borrow)]
    KeyframeRope(KeyframeRope<'a>),
    #[serde(rename = "keyframe_track")]
    #[serde(borrow)]
    KeyframeTrack(KeyframeTrack<'a>),
    #[serde(rename = "light")]
    #[serde(borrow)]
    Light(Light<'a>),
    #[serde(rename = "light_dynamic")]
    #[serde(borrow)]
    LightDynamic(LightDynamic<'a>),
    #[serde(rename = "light_environment")]
    #[serde(borrow)]
    LightEnvironment(LightEnvironment<'a>),
    #[serde(rename = "light_glspot")]
    #[serde(borrow)]
    LightGlspot(LightGlspot<'a>),
    #[serde(rename = "light_spot")]
    #[serde(borrow)]
    LightSpot(LightSpot<'a>),
    #[serde(rename = "localName")]
    LocalName(LocalName),
    #[serde(rename = "logic_achievement")]
    #[serde(borrow)]
    LogicAchievement(LogicAchievement<'a>),
    #[serde(rename = "logic_active_autosave")]
    LogicActiveAutosave(LogicActiveAutosave),
    #[serde(rename = "logic_auto")]
    #[serde(borrow)]
    LogicAuto(LogicAuto<'a>),
    #[serde(rename = "logic_autosave")]
    LogicAutosave(LogicAutosave),
    #[serde(rename = "logic_branch")]
    LogicBranch(LogicBranch),
    #[serde(rename = "logic_branch_listener")]
    #[serde(borrow)]
    LogicBranchListener(LogicBranchListener<'a>),
    #[serde(rename = "logic_case")]
    #[serde(borrow)]
    LogicCase(LogicCase<'a>),
    #[serde(rename = "logic_choreographed_scene")]
    #[serde(borrow)]
    LogicChoreographedScene(LogicChoreographedScene<'a>),
    #[serde(rename = "logic_collision_pair")]
    #[serde(borrow)]
    LogicCollisionPair(LogicCollisionPair<'a>),
    #[serde(rename = "logic_compare")]
    LogicCompare(LogicCompare),
    #[serde(rename = "logic_game_message")]
    #[serde(borrow)]
    LogicGameMessage(LogicGameMessage<'a>),
    #[serde(rename = "logic_lineto")]
    #[serde(borrow)]
    LogicLineto(LogicLineto<'a>),
    #[serde(rename = "logic_measure_movement")]
    #[serde(borrow)]
    LogicMeasureMovement(LogicMeasureMovement<'a>),
    #[serde(rename = "logic_mirror_movement")]
    #[serde(borrow)]
    LogicMirrorMovement(LogicMirrorMovement<'a>),
    #[serde(rename = "logic_multicompare")]
    LogicMulticompare(LogicMulticompare),
    #[serde(rename = "logic_navigation")]
    LogicNavigation(LogicNavigation),
    #[serde(rename = "logic_playerproxy")]
    LogicPlayerproxy(LogicPlayerproxy),
    #[serde(rename = "logic_proximity")]
    #[serde(borrow)]
    LogicProximity(LogicProximity<'a>),
    #[serde(rename = "logic_relay")]
    LogicRelay(LogicRelay),
    #[serde(rename = "logic_scene_list_manager")]
    #[serde(borrow)]
    LogicSceneListManager(LogicSceneListManager<'a>),
    #[serde(rename = "logic_script")]
    #[serde(borrow)]
    LogicScript(LogicScript<'a>),
    #[serde(rename = "logic_timer")]
    LogicTimer(LogicTimer),
    #[serde(rename = "lookdoorthinker")]
    Lookdoorthinker(Lookdoorthinker),
    #[serde(rename = "mapClassName")]
    MapClassName(MapClassName),
    #[serde(rename = "mapobj_cart_dispenser")]
    #[serde(borrow)]
    MapobjCartDispenser(MapobjCartDispenser<'a>),
    #[serde(rename = "material_modify_control")]
    #[serde(borrow)]
    MaterialModifyControl(MaterialModifyControl<'a>),
    #[serde(rename = "math_colorblend")]
    MathColorblend(MathColorblend),
    #[serde(rename = "math_counter")]
    MathCounter(MathCounter),
    #[serde(rename = "math_remap")]
    MathRemap(MathRemap),
    #[serde(rename = "merasmus")]
    Merasmus(Merasmus),
    #[serde(rename = "merasmus_dancer")]
    #[serde(borrow)]
    MerasmusDancer(MerasmusDancer<'a>),
    #[serde(rename = "model_studio")]
    ModelStudio(ModelStudio),
    #[serde(rename = "momentary_door")]
    #[serde(borrow)]
    MomentaryDoor(MomentaryDoor<'a>),
    #[serde(rename = "momentary_rot_button")]
    MomentaryRotButton(MomentaryRotButton),
    #[serde(rename = "monster_furniture")]
    #[serde(borrow)]
    MonsterFurniture(MonsterFurniture<'a>),
    #[serde(rename = "monster_generic")]
    #[serde(borrow)]
    MonsterGeneric(MonsterGeneric<'a>),
    #[serde(rename = "monster_miniturret")]
    MonsterMiniturret(MonsterMiniturret),
    #[serde(rename = "monster_miniturret")]
    MonsterMiniturret(MonsterMiniturret),
    #[serde(rename = "monster_resource")]
    #[serde(borrow)]
    MonsterResource(MonsterResource<'a>),
    #[serde(rename = "monster_sentry")]
    MonsterSentry(MonsterSentry),
    #[serde(rename = "monster_sentry")]
    MonsterSentry(MonsterSentry),
    #[serde(rename = "monster_turret")]
    MonsterTurret(MonsterTurret),
    #[serde(rename = "monster_turret")]
    MonsterTurret(MonsterTurret),
    #[serde(rename = "mortarshell")]
    #[serde(borrow)]
    Mortarshell(Mortarshell<'a>),
    #[serde(rename = "move_keyframed")]
    #[serde(borrow)]
    MoveKeyframed(MoveKeyframed<'a>),
    #[serde(rename = "move_rope")]
    #[serde(borrow)]
    MoveRope(MoveRope<'a>),
    #[serde(rename = "multisource")]
    #[serde(borrow)]
    Multisource(Multisource<'a>),
    #[serde(rename = "my_brush_entity")]
    #[serde(borrow)]
    MyBrushEntity(MyBrushEntity<'a>),
    #[serde(rename = "my_logical_entity")]
    MyLogicalEntity(MyLogicalEntity),
    #[serde(rename = "my_model_entity")]
    #[serde(borrow)]
    MyModelEntity(MyModelEntity<'a>),
    #[serde(rename = "my_NPC")]
    #[serde(borrow)]
    MyNpc(MyNpc<'a>),
    #[serde(rename = "npc_advisor")]
    #[serde(borrow)]
    NpcAdvisor(NpcAdvisor<'a>),
    #[serde(rename = "npc_alyx")]
    NpcAlyx(NpcAlyx),
    #[serde(rename = "npc_alyx")]
    NpcAlyx(NpcAlyx),
    #[serde(rename = "npc_antlion")]
    NpcAntlion(NpcAntlion),
    #[serde(rename = "npc_antlion_grub")]
    #[serde(borrow)]
    NpcAntlionGrub(NpcAntlionGrub<'a>),
    #[serde(rename = "npc_antlion_template_maker")]
    #[serde(borrow)]
    NpcAntlionTemplateMaker(NpcAntlionTemplateMaker<'a>),
    #[serde(rename = "npc_antlionguard")]
    #[serde(borrow)]
    NpcAntlionguard(NpcAntlionguard<'a>),
    #[serde(rename = "npc_apcdriver")]
    #[serde(borrow)]
    NpcApcdriver(NpcApcdriver<'a>),
    #[serde(rename = "npc_assassin")]
    #[serde(borrow)]
    NpcAssassin(NpcAssassin<'a>),
    #[serde(rename = "npc_barnacle")]
    #[serde(borrow)]
    NpcBarnacle(NpcBarnacle<'a>),
    #[serde(rename = "npc_barnacle_tongue_tip")]
    #[serde(borrow)]
    NpcBarnacleTongueTip(NpcBarnacleTongueTip<'a>),
    #[serde(rename = "npc_barney")]
    NpcBarney(NpcBarney),
    #[serde(rename = "npc_blob")]
    #[serde(borrow)]
    NpcBlob(NpcBlob<'a>),
    #[serde(rename = "npc_breen")]
    #[serde(borrow)]
    NpcBreen(NpcBreen<'a>),
    #[serde(rename = "npc_bullseye")]
    #[serde(borrow)]
    NpcBullseye(NpcBullseye<'a>),
    #[serde(rename = "npc_bullsquid")]
    #[serde(borrow)]
    NpcBullsquid(NpcBullsquid<'a>),
    #[serde(rename = "npc_citizen")]
    #[serde(borrow)]
    NpcCitizen(NpcCitizen<'a>),
    #[serde(rename = "npc_clawscanner")]
    NpcClawscanner(NpcClawscanner),
    #[serde(rename = "npc_combine")]
    #[serde(borrow)]
    NpcCombine(NpcCombine<'a>),
    #[serde(rename = "npc_combine_armored")]
    NpcCombineArmored(NpcCombineArmored),
    #[serde(rename = "npc_combine_camera")]
    #[serde(borrow)]
    NpcCombineCamera(NpcCombineCamera<'a>),
    #[serde(rename = "npc_combine_cannon")]
    #[serde(borrow)]
    NpcCombineCannon(NpcCombineCannon<'a>),
    #[serde(rename = "npc_combine_s")]
    NpcCombineS(NpcCombineS),
    #[serde(rename = "npc_combinedropship")]
    #[serde(borrow)]
    NpcCombinedropship(NpcCombinedropship<'a>),
    #[serde(rename = "npc_combinegunship")]
    NpcCombinegunship(NpcCombinegunship),
    #[serde(rename = "npc_concussiongrenade")]
    NpcConcussiongrenade(NpcConcussiongrenade),
    #[serde(rename = "npc_contactgrenade")]
    NpcContactgrenade(NpcContactgrenade),
    #[serde(rename = "npc_cranedriver")]
    #[serde(borrow)]
    NpcCranedriver(NpcCranedriver<'a>),
    #[serde(rename = "npc_crow")]
    #[serde(borrow)]
    NpcCrow(NpcCrow<'a>),
    #[serde(rename = "npc_cscanner")]
    NpcCscanner(NpcCscanner),
    #[serde(rename = "npc_dog")]
    #[serde(borrow)]
    NpcDog(NpcDog<'a>),
    #[serde(rename = "npc_eli")]
    #[serde(borrow)]
    NpcEli(NpcEli<'a>),
    #[serde(rename = "npc_enemyfinder")]
    #[serde(borrow)]
    NpcEnemyfinder(NpcEnemyfinder<'a>),
    #[serde(rename = "npc_enemyfinder_combinecannon")]
    #[serde(borrow)]
    NpcEnemyfinderCombinecannon(NpcEnemyfinderCombinecannon<'a>),
    #[serde(rename = "npc_fastzombie")]
    NpcFastzombie(NpcFastzombie),
    #[serde(rename = "npc_fastzombie_torso")]
    NpcFastzombieTorso(NpcFastzombieTorso),
    #[serde(rename = "npc_fisherman")]
    NpcFisherman(NpcFisherman),
    #[serde(rename = "npc_furniture")]
    #[serde(borrow)]
    NpcFurniture(NpcFurniture<'a>),
    #[serde(rename = "npc_gman")]
    NpcGman(NpcGman),
    #[serde(rename = "npc_grenade_bugbait")]
    NpcGrenadeBugbait(NpcGrenadeBugbait),
    #[serde(rename = "npc_grenade_frag")]
    NpcGrenadeFrag(NpcGrenadeFrag),
    #[serde(rename = "npc_grenade_hopwire")]
    NpcGrenadeHopwire(NpcGrenadeHopwire),
    #[serde(rename = "npc_handgrenade")]
    NpcHandgrenade(NpcHandgrenade),
    #[serde(rename = "npc_headcrab")]
    NpcHeadcrab(NpcHeadcrab),
    #[serde(rename = "npc_headcrab_black")]
    NpcHeadcrabBlack(NpcHeadcrabBlack),
    #[serde(rename = "npc_headcrab_fast")]
    NpcHeadcrabFast(NpcHeadcrabFast),
    #[serde(rename = "npc_headcrab_poison")]
    NpcHeadcrabPoison(NpcHeadcrabPoison),
    #[serde(rename = "npc_heli_avoidbox")]
    #[serde(borrow)]
    NpcHeliAvoidbox(NpcHeliAvoidbox<'a>),
    #[serde(rename = "npc_heli_avoidsphere")]
    #[serde(borrow)]
    NpcHeliAvoidsphere(NpcHeliAvoidsphere<'a>),
    #[serde(rename = "npc_heli_nobomb")]
    #[serde(borrow)]
    NpcHeliNobomb(NpcHeliNobomb<'a>),
    #[serde(rename = "npc_helicopter")]
    #[serde(borrow)]
    NpcHelicopter(NpcHelicopter<'a>),
    #[serde(rename = "npc_helicoptersensor")]
    #[serde(borrow)]
    NpcHelicoptersensor(NpcHelicoptersensor<'a>),
    #[serde(rename = "npc_houndeye")]
    #[serde(borrow)]
    NpcHoundeye(NpcHoundeye<'a>),
    #[serde(rename = "npc_hunter")]
    #[serde(borrow)]
    NpcHunter(NpcHunter<'a>),
    #[serde(rename = "npc_hunter_maker")]
    #[serde(borrow)]
    NpcHunterMaker(NpcHunterMaker<'a>),
    #[serde(rename = "npc_hydra")]
    #[serde(borrow)]
    NpcHydra(NpcHydra<'a>),
    #[serde(rename = "npc_ichthyosaur")]
    #[serde(borrow)]
    NpcIchthyosaur(NpcIchthyosaur<'a>),
    #[serde(rename = "npc_kleiner")]
    #[serde(borrow)]
    NpcKleiner(NpcKleiner<'a>),
    #[serde(rename = "npc_launcher")]
    #[serde(borrow)]
    NpcLauncher(NpcLauncher<'a>),
    #[serde(rename = "npc_magnusson")]
    #[serde(borrow)]
    NpcMagnusson(NpcMagnusson<'a>),
    #[serde(rename = "npc_maker")]
    #[serde(borrow)]
    NpcMaker(NpcMaker<'a>),
    #[serde(rename = "npc_manhack")]
    NpcManhack(NpcManhack),
    #[serde(rename = "npc_metropolice")]
    #[serde(borrow)]
    NpcMetropolice(NpcMetropolice<'a>),
    #[serde(rename = "npc_miniturret")]
    NpcMiniturret(NpcMiniturret),
    #[serde(rename = "npc_missiledefense")]
    #[serde(borrow)]
    NpcMissiledefense(NpcMissiledefense<'a>),
    #[serde(rename = "npc_monk")]
    NpcMonk(NpcMonk),
    #[serde(rename = "npc_mossman")]
    NpcMossman(NpcMossman),
    #[serde(rename = "npc_newnpc")]
    #[serde(borrow)]
    NpcNewnpc(NpcNewnpc<'a>),
    #[serde(rename = "npc_pigeon")]
    NpcPigeon(NpcPigeon),
    #[serde(rename = "npc_poisonzombie")]
    NpcPoisonzombie(NpcPoisonzombie),
    #[serde(rename = "npc_puppet")]
    #[serde(borrow)]
    NpcPuppet(NpcPuppet<'a>),
    #[serde(rename = "npc_rollermine")]
    NpcRollermine(NpcRollermine),
    #[serde(rename = "npc_satchel")]
    NpcSatchel(NpcSatchel),
    #[serde(rename = "npc_satchel")]
    NpcSatchel(NpcSatchel),
    #[serde(rename = "npc_seagull")]
    NpcSeagull(NpcSeagull),
    #[serde(rename = "NPC_sentry")]
    NpcSentry(NpcSentry),
    #[serde(rename = "npc_sniper")]
    #[serde(borrow)]
    NpcSniper(NpcSniper<'a>),
    #[serde(rename = "npc_spotlight")]
    #[serde(borrow)]
    NpcSpotlight(NpcSpotlight<'a>),
    #[serde(rename = "npc_stalker")]
    NpcStalker(NpcStalker),
    #[serde(rename = "npc_strider")]
    NpcStrider(NpcStrider),
    #[serde(rename = "npc_template_maker")]
    #[serde(borrow)]
    NpcTemplateMaker(NpcTemplateMaker<'a>),
    #[serde(rename = "npc_tripmine")]
    NpcTripmine(NpcTripmine),
    #[serde(rename = "npc_tripmine")]
    NpcTripmine(NpcTripmine),
    #[serde(rename = "npc_tripwire")]
    NpcTripwire(NpcTripwire),
    #[serde(rename = "npc_turret_ceiling")]
    NpcTurretCeiling(NpcTurretCeiling),
    #[serde(rename = "npc_turret_ceiling")]
    #[serde(borrow)]
    NpcTurretCeiling(NpcTurretCeiling<'a>),
    #[serde(rename = "npc_turret_floor")]
    NpcTurretFloor(NpcTurretFloor),
    #[serde(rename = "npc_turret_ground")]
    #[serde(borrow)]
    NpcTurretGround(NpcTurretGround<'a>),
    #[serde(rename = "npc_vehicledriver")]
    #[serde(borrow)]
    NpcVehicledriver(NpcVehicledriver<'a>),
    #[serde(rename = "npc_vortigaunt")]
    NpcVortigaunt(NpcVortigaunt),
    #[serde(rename = "npc_wpnscanner")]
    NpcWpnscanner(NpcWpnscanner),
    #[serde(rename = "npc_zombie")]
    NpcZombie(NpcZombie),
    #[serde(rename = "npc_zombie_torso")]
    NpcZombieTorso(NpcZombieTorso),
    #[serde(rename = "npc_zombine")]
    NpcZombine(NpcZombine),
    #[serde(rename = "obj_attachment_sapper")]
    ObjAttachmentSapper(ObjAttachmentSapper),
    #[serde(rename = "obj_dispenser")]
    #[serde(borrow)]
    ObjDispenser(ObjDispenser<'a>),
    #[serde(rename = "obj_sentrygun")]
    ObjSentrygun(ObjSentrygun),
    #[serde(rename = "obj_teleporter")]
    #[serde(borrow)]
    ObjTeleporter(ObjTeleporter<'a>),
    #[serde(rename = "passtime_ball")]
    #[serde(borrow)]
    PasstimeBall(PasstimeBall<'a>),
    #[serde(rename = "passtime_ball")]
    PasstimeBall(PasstimeBall),
    #[serde(rename = "passtime_logic")]
    PasstimeLogic(PasstimeLogic),
    #[serde(rename = "passtime_logic")]
    PasstimeLogic(PasstimeLogic),
    #[serde(rename = "path_corner")]
    PathCorner(PathCorner),
    #[serde(rename = "path_corner_crash")]
    PathCornerCrash(PathCornerCrash),
    #[serde(rename = "path_track")]
    #[serde(borrow)]
    PathTrack(PathTrack<'a>),
    #[serde(rename = "pd_dispenser")]
    PdDispenser(PdDispenser),
    #[serde(rename = "phys_ballsocket")]
    #[serde(borrow)]
    PhysBallsocket(PhysBallsocket<'a>),
    #[serde(rename = "phys_bone_follower")]
    #[serde(borrow)]
    PhysBoneFollower(PhysBoneFollower<'a>),
    #[serde(rename = "phys_constraint")]
    #[serde(borrow)]
    PhysConstraint(PhysConstraint<'a>),
    #[serde(rename = "phys_constraintsystem")]
    PhysConstraintsystem(PhysConstraintsystem),
    #[serde(rename = "phys_convert")]
    #[serde(borrow)]
    PhysConvert(PhysConvert<'a>),
    #[serde(rename = "phys_hinge")]
    #[serde(borrow)]
    PhysHinge(PhysHinge<'a>),
    #[serde(rename = "phys_keepupright")]
    #[serde(borrow)]
    PhysKeepupright(PhysKeepupright<'a>),
    #[serde(rename = "phys_lengthconstraint")]
    #[serde(borrow)]
    PhysLengthconstraint(PhysLengthconstraint<'a>),
    #[serde(rename = "phys_magnet")]
    #[serde(borrow)]
    PhysMagnet(PhysMagnet<'a>),
    #[serde(rename = "phys_motor")]
    #[serde(borrow)]
    PhysMotor(PhysMotor<'a>),
    #[serde(rename = "phys_pulleyconstraint")]
    #[serde(borrow)]
    PhysPulleyconstraint(PhysPulleyconstraint<'a>),
    #[serde(rename = "phys_ragdollconstraint")]
    #[serde(borrow)]
    PhysRagdollconstraint(PhysRagdollconstraint<'a>),
    #[serde(rename = "phys_ragdollmagnet")]
    PhysRagdollmagnet(PhysRagdollmagnet),
    #[serde(rename = "phys_slideconstraint")]
    #[serde(borrow)]
    PhysSlideconstraint(PhysSlideconstraint<'a>),
    #[serde(rename = "phys_spring")]
    #[serde(borrow)]
    PhysSpring(PhysSpring<'a>),
    #[serde(rename = "phys_thruster")]
    #[serde(borrow)]
    PhysThruster(PhysThruster<'a>),
    #[serde(rename = "phys_torque")]
    #[serde(borrow)]
    PhysTorque(PhysTorque<'a>),
    #[serde(rename = "physics_cannister")]
    #[serde(borrow)]
    PhysicsCannister(PhysicsCannister<'a>),
    #[serde(rename = "physics_entity_solver")]
    PhysicsEntitySolver(PhysicsEntitySolver),
    #[serde(rename = "physics_npc_solver")]
    PhysicsNpcSolver(PhysicsNpcSolver),
    #[serde(rename = "physics_prop")]
    #[serde(borrow)]
    PhysicsProp(PhysicsProp<'a>),
    #[serde(rename = "physics_prop_ragdoll")]
    #[serde(borrow)]
    PhysicsPropRagdoll(PhysicsPropRagdoll<'a>),
    #[serde(rename = "_plasma")]
    Plasma(Plasma),
    #[serde(rename = "player")]
    Player(Player),
    #[serde(rename = "player")]
    Player(Player),
    #[serde(rename = "player")]
    Player(Player),
    #[serde(rename = "player")]
    Player(Player),
    #[serde(rename = "player")]
    Player(Player),
    #[serde(rename = "player")]
    Player(Player),
    #[serde(rename = "player_loadsaved")]
    #[serde(borrow)]
    PlayerLoadsaved(PlayerLoadsaved<'a>),
    #[serde(rename = "player_manager")]
    #[serde(borrow)]
    PlayerManager(PlayerManager<'a>),
    #[serde(rename = "player_pickup")]
    #[serde(borrow)]
    PlayerPickup(PlayerPickup<'a>),
    #[serde(rename = "player_pickup")]
    #[serde(borrow)]
    PlayerPickup(PlayerPickup<'a>),
    #[serde(rename = "player_speedmod")]
    PlayerSpeedmod(PlayerSpeedmod),
    #[serde(rename = "player_weaponstrip")]
    PlayerWeaponstrip(PlayerWeaponstrip),
    #[serde(rename = "point_anglesensor")]
    #[serde(borrow)]
    PointAnglesensor(PointAnglesensor<'a>),
    #[serde(rename = "point_angularvelocitysensor")]
    PointAngularvelocitysensor(PointAngularvelocitysensor),
    #[serde(rename = "point_antlion_repellant")]
    PointAntlionRepellant(PointAntlionRepellant),
    #[serde(rename = "point_apc_controller")]
    #[serde(borrow)]
    PointApcController(PointApcController<'a>),
    #[serde(rename = "point_bonusmaps_accessor")]
    #[serde(borrow)]
    PointBonusmapsAccessor(PointBonusmapsAccessor<'a>),
    #[serde(rename = "point_bugbait")]
    PointBugbait(PointBugbait),
    #[serde(rename = "point_camera")]
    #[serde(borrow)]
    PointCamera(PointCamera<'a>),
    #[serde(rename = "point_clientcommand")]
    PointClientcommand(PointClientcommand),
    #[serde(rename = "point_combine_ball_launcher")]
    #[serde(borrow)]
    PointCombineBallLauncher(PointCombineBallLauncher<'a>),
    #[serde(rename = "point_commentary_node")]
    #[serde(borrow)]
    PointCommentaryNode(PointCommentaryNode<'a>),
    #[serde(rename = "point_commentary_viewpoint")]
    PointCommentaryViewpoint(PointCommentaryViewpoint),
    #[serde(rename = "point_devshot_camera")]
    #[serde(borrow)]
    PointDevshotCamera(PointDevshotCamera<'a>),
    #[serde(rename = "point_enable_motion_fixup")]
    #[serde(borrow)]
    PointEnableMotionFixup(PointEnableMotionFixup<'a>),
    #[serde(rename = "point_flesh_effect_target")]
    PointFleshEffectTarget(PointFleshEffectTarget),
    #[serde(rename = "point_gamestats_counter")]
    #[serde(borrow)]
    PointGamestatsCounter(PointGamestatsCounter<'a>),
    #[serde(rename = "point_gamestats_counter")]
    #[serde(borrow)]
    PointGamestatsCounter(PointGamestatsCounter<'a>),
    #[serde(rename = "point_hurt")]
    #[serde(borrow)]
    PointHurt(PointHurt<'a>),
    #[serde(rename = "point_intermission")]
    PointIntermission(PointIntermission),
    #[serde(rename = "point_message")]
    #[serde(borrow)]
    PointMessage(PointMessage<'a>),
    #[serde(rename = "point_playermoveconstraint")]
    #[serde(borrow)]
    PointPlayermoveconstraint(PointPlayermoveconstraint<'a>),
    #[serde(rename = "point_populator_interface")]
    PointPopulatorInterface(PointPopulatorInterface),
    #[serde(rename = "point_posecontroller")]
    #[serde(borrow)]
    PointPosecontroller(PointPosecontroller<'a>),
    #[serde(rename = "point_proximity_sensor")]
    PointProximitySensor(PointProximitySensor),
    #[serde(rename = "point_push")]
    PointPush(PointPush),
    #[serde(rename = "point_script_template")]
    #[serde(borrow)]
    PointScriptTemplate(PointScriptTemplate<'a>),
    #[serde(rename = "point_servercommand")]
    PointServercommand(PointServercommand),
    #[serde(rename = "point_spotlight")]
    PointSpotlight(PointSpotlight),
    #[serde(rename = "point_teleport")]
    #[serde(borrow)]
    PointTeleport(PointTeleport<'a>),
    #[serde(rename = "point_template")]
    #[serde(borrow)]
    PointTemplate(PointTemplate<'a>),
    #[serde(rename = "point_tesla")]
    #[serde(borrow)]
    PointTesla(PointTesla<'a>),
    #[serde(rename = "point_velocitysensor")]
    PointVelocitysensor(PointVelocitysensor),
    #[serde(rename = "point_viewcontrol")]
    #[serde(borrow)]
    PointViewcontrol(PointViewcontrol<'a>),
    #[serde(rename = "point_worldtext")]
    #[serde(borrow)]
    PointWorldtext(PointWorldtext<'a>),
    #[serde(rename = "point_worldtext")]
    PointWorldtext(PointWorldtext),
    #[serde(rename = "populator_internal_spawn_point")]
    PopulatorInternalSpawnPoint(PopulatorInternalSpawnPoint),
    #[serde(rename = "portable_thumper")]
    #[serde(borrow)]
    PortableThumper(PortableThumper<'a>),
    #[serde(rename = "predicted_viewmodel")]
    PredictedViewmodel(PredictedViewmodel),
    #[serde(rename = "prop_combine_ball")]
    #[serde(borrow)]
    PropCombineBall(PropCombineBall<'a>),
    #[serde(rename = "prop_coreball")]
    #[serde(borrow)]
    PropCoreball(PropCoreball<'a>),
    #[serde(rename = "prop_door_rotating")]
    #[serde(borrow)]
    PropDoorRotating(PropDoorRotating<'a>),
    #[serde(rename = "prop_dropship_container")]
    #[serde(borrow)]
    PropDropshipContainer(PropDropshipContainer<'a>),
    #[serde(rename = "prop_dynamic")]
    #[serde(borrow)]
    PropDynamic(PropDynamic<'a>),
    #[serde(rename = "prop_dynamic_ornament")]
    #[serde(borrow)]
    PropDynamicOrnament(PropDynamicOrnament<'a>),
    #[serde(rename = "prop_dynamic_override")]
    #[serde(borrow)]
    PropDynamicOverride(PropDynamicOverride<'a>),
    #[serde(rename = "prop_physics")]
    #[serde(borrow)]
    PropPhysics(PropPhysics<'a>),
    #[serde(rename = "prop_physics_multiplayer")]
    #[serde(borrow)]
    PropPhysicsMultiplayer(PropPhysicsMultiplayer<'a>),
    #[serde(rename = "prop_physics_override")]
    #[serde(borrow)]
    PropPhysicsOverride(PropPhysicsOverride<'a>),
    #[serde(rename = "prop_physics_respawnable")]
    #[serde(borrow)]
    PropPhysicsRespawnable(PropPhysicsRespawnable<'a>),
    #[serde(rename = "prop_ragdoll")]
    #[serde(borrow)]
    PropRagdoll(PropRagdoll<'a>),
    #[serde(rename = "prop_ragdoll_attached")]
    #[serde(borrow)]
    PropRagdollAttached(PropRagdollAttached<'a>),
    #[serde(rename = "prop_scalable")]
    #[serde(borrow)]
    PropScalable(PropScalable<'a>),
    #[serde(rename = "prop_soccer_ball")]
    #[serde(borrow)]
    PropSoccerBall(PropSoccerBall<'a>),
    #[serde(rename = "prop_sphere")]
    #[serde(borrow)]
    PropSphere(PropSphere<'a>),
    #[serde(rename = "prop_stickybomb")]
    #[serde(borrow)]
    PropStickybomb(PropStickybomb<'a>),
    #[serde(rename = "prop_thumper")]
    #[serde(borrow)]
    PropThumper(PropThumper<'a>),
    #[serde(rename = "prop_vehicle")]
    #[serde(borrow)]
    PropVehicle(PropVehicle<'a>),
    #[serde(rename = "prop_vehicle_airboat")]
    PropVehicleAirboat(PropVehicleAirboat),
    #[serde(rename = "prop_vehicle_apc")]
    #[serde(borrow)]
    PropVehicleApc(PropVehicleApc<'a>),
    #[serde(rename = "prop_vehicle_cannon")]
    #[serde(borrow)]
    PropVehicleCannon(PropVehicleCannon<'a>),
    #[serde(rename = "prop_vehicle_choreo_generic")]
    #[serde(borrow)]
    PropVehicleChoreoGeneric(PropVehicleChoreoGeneric<'a>),
    #[serde(rename = "prop_vehicle_crane")]
    #[serde(borrow)]
    PropVehicleCrane(PropVehicleCrane<'a>),
    #[serde(rename = "prop_vehicle_driveable")]
    #[serde(borrow)]
    PropVehicleDriveable(PropVehicleDriveable<'a>),
    #[serde(rename = "prop_vehicle_jeep")]
    PropVehicleJeep(PropVehicleJeep),
    #[serde(rename = "prop_vehicle_jeep")]
    PropVehicleJeep(PropVehicleJeep),
    #[serde(rename = "prop_vehicle_jeep")]
    PropVehicleJeep(PropVehicleJeep),
    #[serde(rename = "prop_vehicle_jetski")]
    PropVehicleJetski(PropVehicleJetski),
    #[serde(rename = "prop_vehicle_prisoner_pod")]
    #[serde(borrow)]
    PropVehiclePrisonerPod(PropVehiclePrisonerPod<'a>),
    #[serde(rename = "proto_sniper")]
    #[serde(borrow)]
    ProtoSniper(ProtoSniper<'a>),
    #[serde(rename = "raggib")]
    #[serde(borrow)]
    Raggib(Raggib<'a>),
    #[serde(rename = "rd_robot_dispenser")]
    RdRobotDispenser(RdRobotDispenser),
    #[serde(rename = "reserved_spot")]
    #[serde(borrow)]
    ReservedSpot(ReservedSpot<'a>),
    #[serde(rename = "rope_anchor")]
    RopeAnchor(RopeAnchor),
    #[serde(rename = "rpg_missile")]
    #[serde(borrow)]
    RpgMissile(RpgMissile<'a>),
    #[serde(rename = "rpg_missile")]
    #[serde(borrow)]
    RpgMissile(RpgMissile<'a>),
    #[serde(rename = "scene_manager")]
    #[serde(borrow)]
    SceneManager(SceneManager<'a>),
    #[serde(rename = "script_intro")]
    #[serde(borrow)]
    ScriptIntro(ScriptIntro<'a>),
    #[serde(rename = "scripted_scene")]
    #[serde(borrow)]
    ScriptedScene(ScriptedScene<'a>),
    #[serde(rename = "scripted_sentence")]
    #[serde(borrow)]
    ScriptedSentence(ScriptedSentence<'a>),
    #[serde(rename = "scripted_sequence")]
    #[serde(borrow)]
    ScriptedSequence(ScriptedSequence<'a>),
    #[serde(rename = "scripted_target")]
    #[serde(borrow)]
    ScriptedTarget(ScriptedTarget<'a>),
    #[serde(rename = "sdk_bot")]
    SdkBot(SdkBot),
    #[serde(rename = "sdk_gamerules")]
    SdkGamerules(SdkGamerules),
    #[serde(rename = "sdk_ragdoll")]
    SdkRagdoll(SdkRagdoll),
    #[serde(rename = "sdk_team_manager")]
    SdkTeamManager(SdkTeamManager),
    #[serde(rename = "shadow_control")]
    #[serde(borrow)]
    ShadowControl(ShadowControl<'a>),
    #[serde(rename = "simple_bot")]
    SimpleBot(SimpleBot),
    #[serde(rename = "simple_physics_brush")]
    #[serde(borrow)]
    SimplePhysicsBrush(SimplePhysicsBrush<'a>),
    #[serde(rename = "simple_physics_prop")]
    SimplePhysicsProp(SimplePhysicsProp),
    #[serde(rename = "sky_camera")]
    SkyCamera(SkyCamera),
    #[serde(rename = "skybox_swapper")]
    #[serde(borrow)]
    SkyboxSwapper(SkyboxSwapper<'a>),
    #[serde(rename = "sniperbullet")]
    #[serde(borrow)]
    Sniperbullet(Sniperbullet<'a>),
    #[serde(rename = "soundent")]
    Soundent(Soundent),
    #[serde(rename = "spark_shower")]
    SparkShower(SparkShower),
    #[serde(rename = "sparktrail")]
    Sparktrail(Sparktrail),
    #[serde(rename = "spotlight_end")]
    #[serde(borrow)]
    SpotlightEnd(SpotlightEnd<'a>),
    #[serde(rename = "spraycan")]
    Spraycan(Spraycan),
    #[serde(rename = "squadinsignia")]
    #[serde(borrow)]
    Squadinsignia(Squadinsignia<'a>),
    #[serde(rename = "tank_boss")]
    #[serde(borrow)]
    TankBoss(TankBoss<'a>),
    #[serde(rename = "tank_boss")]
    TankBoss(TankBoss),
    #[serde(rename = "tank_destruction")]
    #[serde(borrow)]
    TankDestruction(TankDestruction<'a>),
    #[serde(rename = "tanktrain_ai")]
    #[serde(borrow)]
    TanktrainAi(TanktrainAi<'a>),
    #[serde(rename = "tanktrain_aitarget")]
    #[serde(borrow)]
    TanktrainAitarget(TanktrainAitarget<'a>),
    #[serde(rename = "target_cdaudio")]
    TargetCdaudio(TargetCdaudio),
    #[serde(rename = "target_changegravity")]
    TargetChangegravity(TargetChangegravity),
    #[serde(rename = "te_tester")]
    TeTester(TeTester),
    #[serde(rename = "team_control_point")]
    #[serde(borrow)]
    TeamControlPoint(TeamControlPoint<'a>),
    #[serde(rename = "team_control_point_master")]
    #[serde(borrow)]
    TeamControlPointMaster(TeamControlPointMaster<'a>),
    #[serde(rename = "team_control_point_round")]
    #[serde(borrow)]
    TeamControlPointRound(TeamControlPointRound<'a>),
    #[serde(rename = "team_manager")]
    #[serde(borrow)]
    TeamManager(TeamManager<'a>),
    #[serde(rename = "team_round_timer")]
    #[serde(borrow)]
    TeamRoundTimer(TeamRoundTimer<'a>),
    #[serde(rename = "team_train_watcher")]
    #[serde(borrow)]
    TeamTrainWatcher(TeamTrainWatcher<'a>),
    #[serde(rename = "team_train_watcher_master")]
    TeamTrainWatcherMaster(TeamTrainWatcherMaster),
    #[serde(rename = "teleport_vortex")]
    #[serde(borrow)]
    TeleportVortex(TeleportVortex<'a>),
    #[serde(rename = "test_effect")]
    #[serde(borrow)]
    TestEffect(TestEffect<'a>),
    #[serde(rename = "test_proxytoggle")]
    #[serde(borrow)]
    TestProxytoggle(TestProxytoggle<'a>),
    #[serde(rename = "test_traceline")]
    TestTraceline(TestTraceline),
    #[serde(rename = "tf_ammo_pack")]
    TfAmmoPack(TfAmmoPack),
    #[serde(rename = "tf_base_minigame")]
    #[serde(borrow)]
    TfBaseMinigame(TfBaseMinigame<'a>),
    #[serde(rename = "tf_bonus_duck_pickup")]
    #[serde(borrow)]
    TfBonusDuckPickup(TfBonusDuckPickup<'a>),
    #[serde(rename = "tf_bot")]
    TfBot(TfBot),
    #[serde(rename = "tf_dropped_weapon")]
    #[serde(borrow)]
    TfDroppedWeapon(TfDroppedWeapon<'a>),
    #[serde(rename = "tf_flame")]
    #[serde(borrow)]
    TfFlame(TfFlame<'a>),
    #[serde(rename = "tf_flame_manager")]
    TfFlameManager(TfFlameManager),
    #[serde(rename = "tf_gamerules")]
    TfGamerules(TfGamerules),
    #[serde(rename = "tf_gas_grenade_effect")]
    #[serde(borrow)]
    TfGasGrenadeEffect(TfGasGrenadeEffect<'a>),
    #[serde(rename = "tf_gas_manager")]
    TfGasManager(TfGasManager),
    #[serde(rename = "tf_generic_bomb")]
    #[serde(borrow)]
    TfGenericBomb(TfGenericBomb<'a>),
    #[serde(rename = "tf_glow")]
    #[serde(borrow)]
    TfGlow(TfGlow<'a>),
    #[serde(rename = "tf_halloween_gift_pickup")]
    #[serde(borrow)]
    TfHalloweenGiftPickup(TfHalloweenGiftPickup<'a>),
    #[serde(rename = "tf_halloween_gift_spawn_location")]
    #[serde(borrow)]
    TfHalloweenGiftSpawnLocation(TfHalloweenGiftSpawnLocation<'a>),
    #[serde(rename = "tf_halloween_item_pickup")]
    TfHalloweenItemPickup(TfHalloweenItemPickup),
    #[serde(rename = "tf_halloween_minigame")]
    #[serde(borrow)]
    TfHalloweenMinigame(TfHalloweenMinigame<'a>),
    #[serde(rename = "tf_halloween_minigame_falling_platforms")]
    TfHalloweenMinigameFallingPlatforms(TfHalloweenMinigameFallingPlatforms),
    #[serde(rename = "tf_halloween_pickup")]
    #[serde(borrow)]
    TfHalloweenPickup(TfHalloweenPickup<'a>),
    #[serde(rename = "tf_logic_arena")]
    TfLogicArena(TfLogicArena),
    #[serde(rename = "tf_logic_bonusround")]
    #[serde(borrow)]
    TfLogicBonusround(TfLogicBonusround<'a>),
    #[serde(rename = "tf_logic_boss_battle")]
    TfLogicBossBattle(TfLogicBossBattle),
    #[serde(rename = "tf_logic_competitive")]
    TfLogicCompetitive(TfLogicCompetitive),
    #[serde(rename = "tf_logic_cp_timer")]
    #[serde(borrow)]
    TfLogicCpTimer(TfLogicCpTimer<'a>),
    #[serde(rename = "tf_logic_holiday")]
    TfLogicHoliday(TfLogicHoliday),
    #[serde(rename = "tf_logic_hybrid_ctf_cp")]
    TfLogicHybridCtfCp(TfLogicHybridCtfCp),
    #[serde(rename = "tf_logic_koth")]
    TfLogicKoth(TfLogicKoth),
    #[serde(rename = "tf_logic_mann_vs_machine")]
    TfLogicMannVsMachine(TfLogicMannVsMachine),
    #[serde(rename = "tf_logic_mannpower")]
    TfLogicMannpower(TfLogicMannpower),
    #[serde(rename = "tf_logic_medieval")]
    TfLogicMedieval(TfLogicMedieval),
    #[serde(rename = "tf_logic_minigames")]
    #[serde(borrow)]
    TfLogicMinigames(TfLogicMinigames<'a>),
    #[serde(rename = "tf_logic_multiple_escort")]
    TfLogicMultipleEscort(TfLogicMultipleEscort),
    #[serde(rename = "tf_logic_on_holiday")]
    TfLogicOnHoliday(TfLogicOnHoliday),
    #[serde(rename = "tf_logic_player_destruction")]
    #[serde(borrow)]
    TfLogicPlayerDestruction(TfLogicPlayerDestruction<'a>),
    #[serde(rename = "tf_logic_raid")]
    TfLogicRaid(TfLogicRaid),
    #[serde(rename = "tf_logic_robot_destruction")]
    #[serde(borrow)]
    TfLogicRobotDestruction(TfLogicRobotDestruction<'a>),
    #[serde(rename = "tf_logic_training_mode")]
    #[serde(borrow)]
    TfLogicTrainingMode(TfLogicTrainingMode<'a>),
    #[serde(rename = "tf_mann_vs_machine_stats")]
    #[serde(borrow)]
    TfMannVsMachineStats(TfMannVsMachineStats<'a>),
    #[serde(rename = "tf_merasmus_trick_or_treat_prop")]
    #[serde(borrow)]
    TfMerasmusTrickOrTreatProp(TfMerasmusTrickOrTreatProp<'a>),
    #[serde(rename = "tf_objective_resource")]
    TfObjectiveResource(TfObjectiveResource),
    #[serde(rename = "tf_pda_expansion_dispenser")]
    TfPdaExpansionDispenser(TfPdaExpansionDispenser),
    #[serde(rename = "tf_pda_expansion_teleporter")]
    TfPdaExpansionTeleporter(TfPdaExpansionTeleporter),
    #[serde(rename = "tf_player_manager")]
    TfPlayerManager(TfPlayerManager),
    #[serde(rename = "tf_point_nav_interface")]
    TfPointNavInterface(TfPointNavInterface),
    #[serde(rename = "tf_point_weapon_mimic")]
    #[serde(borrow)]
    TfPointWeaponMimic(TfPointWeaponMimic<'a>),
    #[serde(rename = "tf_powerup_bottle")]
    TfPowerupBottle(TfPowerupBottle),
    #[serde(rename = "tf_projectile_arrow")]
    TfProjectileArrow(TfProjectileArrow),
    #[serde(rename = "tf_projectile_ball_ornament")]
    TfProjectileBallOrnament(TfProjectileBallOrnament),
    #[serde(rename = "tf_projectile_balloffire")]
    TfProjectileBalloffire(TfProjectileBalloffire),
    #[serde(rename = "tf_projectile_cleaver")]
    TfProjectileCleaver(TfProjectileCleaver),
    #[serde(rename = "tf_projectile_energy_ball")]
    TfProjectileEnergyBall(TfProjectileEnergyBall),
    #[serde(rename = "tf_projectile_energy_ring")]
    TfProjectileEnergyRing(TfProjectileEnergyRing),
    #[serde(rename = "tf_projectile_flare")]
    TfProjectileFlare(TfProjectileFlare),
    #[serde(rename = "tf_projectile_grapplinghook")]
    TfProjectileGrapplinghook(TfProjectileGrapplinghook),
    #[serde(rename = "tf_projectile_healing_bolt")]
    TfProjectileHealingBolt(TfProjectileHealingBolt),
    #[serde(rename = "tf_projectile_jar")]
    TfProjectileJar(TfProjectileJar),
    #[serde(rename = "tf_projectile_jar_gas")]
    TfProjectileJarGas(TfProjectileJarGas),
    #[serde(rename = "tf_projectile_jar_milk")]
    TfProjectileJarMilk(TfProjectileJarMilk),
    #[serde(rename = "tf_projectile_lightningorb")]
    TfProjectileLightningorb(TfProjectileLightningorb),
    #[serde(rename = "tf_projectile_mechanicalarmorb")]
    TfProjectileMechanicalarmorb(TfProjectileMechanicalarmorb),
    #[serde(rename = "tf_projectile_pipe")]
    TfProjectilePipe(TfProjectilePipe),
    #[serde(rename = "tf_projectile_pipe_remote")]
    TfProjectilePipeRemote(TfProjectilePipeRemote),
    #[serde(rename = "tf_projectile_rocket")]
    TfProjectileRocket(TfProjectileRocket),
    #[serde(rename = "tf_projectile_sentryrocket")]
    TfProjectileSentryrocket(TfProjectileSentryrocket),
    #[serde(rename = "tf_projectile_spellbats")]
    TfProjectileSpellbats(TfProjectileSpellbats),
    #[serde(rename = "tf_projectile_spellfireball")]
    TfProjectileSpellfireball(TfProjectileSpellfireball),
    #[serde(rename = "tf_projectile_spellkartbats")]
    TfProjectileSpellkartbats(TfProjectileSpellkartbats),
    #[serde(rename = "tf_projectile_spellkartmirv")]
    TfProjectileSpellkartmirv(TfProjectileSpellkartmirv),
    #[serde(rename = "tf_projectile_spellkartorb")]
    TfProjectileSpellkartorb(TfProjectileSpellkartorb),
    #[serde(rename = "tf_projectile_spellkartpumpkin")]
    TfProjectileSpellkartpumpkin(TfProjectileSpellkartpumpkin),
    #[serde(rename = "tf_projectile_spellmeteorshower")]
    TfProjectileSpellmeteorshower(TfProjectileSpellmeteorshower),
    #[serde(rename = "tf_projectile_spellmirv")]
    TfProjectileSpellmirv(TfProjectileSpellmirv),
    #[serde(rename = "tf_projectile_spellpumpkin")]
    TfProjectileSpellpumpkin(TfProjectileSpellpumpkin),
    #[serde(rename = "tf_projectile_spellspawnboss")]
    TfProjectileSpellspawnboss(TfProjectileSpellspawnboss),
    #[serde(rename = "tf_projectile_spellspawnhorde")]
    TfProjectileSpellspawnhorde(TfProjectileSpellspawnhorde),
    #[serde(rename = "tf_projectile_spellspawnzombie")]
    TfProjectileSpellspawnzombie(TfProjectileSpellspawnzombie),
    #[serde(rename = "tf_projectile_spelltransposeteleport")]
    TfProjectileSpelltransposeteleport(TfProjectileSpelltransposeteleport),
    #[serde(rename = "tf_projectile_stun_ball")]
    TfProjectileStunBall(TfProjectileStunBall),
    #[serde(rename = "tf_projectile_syringe")]
    TfProjectileSyringe(TfProjectileSyringe),
    #[serde(rename = "tf_projectile_throwable")]
    TfProjectileThrowable(TfProjectileThrowable),
    #[serde(rename = "tf_projectile_throwable_breadmonster")]
    TfProjectileThrowableBreadmonster(TfProjectileThrowableBreadmonster),
    #[serde(rename = "tf_projectile_throwable_brick")]
    TfProjectileThrowableBrick(TfProjectileThrowableBrick),
    #[serde(rename = "tf_projectile_throwable_repel")]
    TfProjectileThrowableRepel(TfProjectileThrowableRepel),
    #[serde(rename = "tf_pumpkin_bomb")]
    TfPumpkinBomb(TfPumpkinBomb),
    #[serde(rename = "tf_ragdoll")]
    TfRagdoll(TfRagdoll),
    #[serde(rename = "tf_robot_destruction_robot")]
    TfRobotDestructionRobot(TfRobotDestructionRobot),
    #[serde(rename = "tf_robot_destruction_robot_spawn")]
    #[serde(borrow)]
    TfRobotDestructionRobotSpawn(TfRobotDestructionRobotSpawn<'a>),
    #[serde(rename = "tf_robot_destruction_spawn_group")]
    #[serde(borrow)]
    TfRobotDestructionSpawnGroup(TfRobotDestructionSpawnGroup<'a>),
    #[serde(rename = "tf_spawner")]
    #[serde(borrow)]
    TfSpawner(TfSpawner<'a>),
    #[serde(rename = "tf_spawner_boss")]
    #[serde(borrow)]
    TfSpawnerBoss(TfSpawnerBoss<'a>),
    #[serde(rename = "tf_spell_meteorshowerspawner")]
    #[serde(borrow)]
    TfSpellMeteorshowerspawner(TfSpellMeteorshowerspawner<'a>),
    #[serde(rename = "tf_spell_pickup")]
    #[serde(borrow)]
    TfSpellPickup(TfSpellPickup<'a>),
    #[serde(rename = "tf_target_dummy")]
    #[serde(borrow)]
    TfTargetDummy(TfTargetDummy<'a>),
    #[serde(rename = "tf_taunt_prop")]
    #[serde(borrow)]
    TfTauntProp(TfTauntProp<'a>),
    #[serde(rename = "tf_team")]
    TfTeam(TfTeam),
    #[serde(rename = "tf_teleport_location")]
    TfTeleportLocation(TfTeleportLocation),
    #[serde(rename = "tf_template_stun_drone")]
    TfTemplateStunDrone(TfTemplateStunDrone),
    #[serde(rename = "tf_viewmodel")]
    TfViewmodel(TfViewmodel),
    #[serde(rename = "tf_weapon_base")]
    TfWeaponBase(TfWeaponBase),
    #[serde(rename = "tf_weapon_bat")]
    TfWeaponBat(TfWeaponBat),
    #[serde(rename = "tf_weapon_bat_fish")]
    TfWeaponBatFish(TfWeaponBatFish),
    #[serde(rename = "tf_weapon_bat_giftwrap")]
    TfWeaponBatGiftwrap(TfWeaponBatGiftwrap),
    #[serde(rename = "tf_weapon_bat_wood")]
    TfWeaponBatWood(TfWeaponBatWood),
    #[serde(rename = "tf_weapon_bonesaw")]
    TfWeaponBonesaw(TfWeaponBonesaw),
    #[serde(rename = "tf_weapon_boomerang")]
    TfWeaponBoomerang(TfWeaponBoomerang),
    #[serde(rename = "tf_weapon_bottle")]
    TfWeaponBottle(TfWeaponBottle),
    #[serde(rename = "tf_weapon_breakable_sign")]
    TfWeaponBreakableSign(TfWeaponBreakableSign),
    #[serde(rename = "tf_weapon_buff_item")]
    TfWeaponBuffItem(TfWeaponBuffItem),
    #[serde(rename = "tf_weapon_builder")]
    TfWeaponBuilder(TfWeaponBuilder),
    #[serde(rename = "tf_weapon_charged_smg")]
    TfWeaponChargedSmg(TfWeaponChargedSmg),
    #[serde(rename = "tf_weapon_cleaver")]
    TfWeaponCleaver(TfWeaponCleaver),
    #[serde(rename = "tf_weapon_club")]
    TfWeaponClub(TfWeaponClub),
    #[serde(rename = "tf_weapon_compound_bow")]
    TfWeaponCompoundBow(TfWeaponCompoundBow),
    #[serde(rename = "tf_weapon_crossbow")]
    TfWeaponCrossbow(TfWeaponCrossbow),
    #[serde(rename = "tf_weapon_crowbar")]
    TfWeaponCrowbar(TfWeaponCrowbar),
    #[serde(rename = "tf_weapon_decoy")]
    TfWeaponDecoy(TfWeaponDecoy),
    #[serde(rename = "tf_weapon_drg_pomson")]
    TfWeaponDrgPomson(TfWeaponDrgPomson),
    #[serde(rename = "tf_weapon_drg_pomson")]
    TfWeaponDrgPomson(TfWeaponDrgPomson),
    #[serde(rename = "tf_weapon_fireaxe")]
    TfWeaponFireaxe(TfWeaponFireaxe),
    #[serde(rename = "tf_weapon_fists")]
    TfWeaponFists(TfWeaponFists),
    #[serde(rename = "tf_weapon_flag")]
    TfWeaponFlag(TfWeaponFlag),
    #[serde(rename = "tf_weapon_flamethrower")]
    TfWeaponFlamethrower(TfWeaponFlamethrower),
    #[serde(rename = "tf_weapon_flaregun")]
    TfWeaponFlaregun(TfWeaponFlaregun),
    #[serde(rename = "tf_weapon_flaregun_revenge")]
    TfWeaponFlaregunRevenge(TfWeaponFlaregunRevenge),
    #[serde(rename = "tf_weapon_grapplinghook")]
    TfWeaponGrapplinghook(TfWeaponGrapplinghook),
    #[serde(rename = "tf_weapon_grenade_caltrop")]
    TfWeaponGrenadeCaltrop(TfWeaponGrenadeCaltrop),
    #[serde(rename = "tf_weapon_grenade_caltrop_projectile")]
    TfWeaponGrenadeCaltropProjectile(TfWeaponGrenadeCaltropProjectile),
    #[serde(rename = "tf_weapon_grenade_concussion")]
    TfWeaponGrenadeConcussion(TfWeaponGrenadeConcussion),
    #[serde(rename = "tf_weapon_grenade_concussion_projectile")]
    TfWeaponGrenadeConcussionProjectile(TfWeaponGrenadeConcussionProjectile),
    #[serde(rename = "tf_weapon_grenade_emp")]
    TfWeaponGrenadeEmp(TfWeaponGrenadeEmp),
    #[serde(rename = "tf_weapon_grenade_emp_projectile")]
    TfWeaponGrenadeEmpProjectile(TfWeaponGrenadeEmpProjectile),
    #[serde(rename = "tf_weapon_grenade_gas")]
    TfWeaponGrenadeGas(TfWeaponGrenadeGas),
    #[serde(rename = "tf_weapon_grenade_gas_projectile")]
    TfWeaponGrenadeGasProjectile(TfWeaponGrenadeGasProjectile),
    #[serde(rename = "tf_weapon_grenade_heal")]
    TfWeaponGrenadeHeal(TfWeaponGrenadeHeal),
    #[serde(rename = "tf_weapon_grenade_heal_projectile")]
    TfWeaponGrenadeHealProjectile(TfWeaponGrenadeHealProjectile),
    #[serde(rename = "tf_weapon_grenade_mirv")]
    TfWeaponGrenadeMirv(TfWeaponGrenadeMirv),
    #[serde(rename = "tf_weapon_grenade_mirv_bomb")]
    TfWeaponGrenadeMirvBomb(TfWeaponGrenadeMirvBomb),
    #[serde(rename = "tf_weapon_grenade_mirv_demoman")]
    TfWeaponGrenadeMirvDemoman(TfWeaponGrenadeMirvDemoman),
    #[serde(rename = "tf_weapon_grenade_mirv_projectile")]
    TfWeaponGrenadeMirvProjectile(TfWeaponGrenadeMirvProjectile),
    #[serde(rename = "tf_weapon_grenade_nail")]
    TfWeaponGrenadeNail(TfWeaponGrenadeNail),
    #[serde(rename = "tf_weapon_grenade_nail_projectile")]
    TfWeaponGrenadeNailProjectile(TfWeaponGrenadeNailProjectile),
    #[serde(rename = "tf_weapon_grenade_napalm")]
    TfWeaponGrenadeNapalm(TfWeaponGrenadeNapalm),
    #[serde(rename = "tf_weapon_grenade_napalm_projectile")]
    TfWeaponGrenadeNapalmProjectile(TfWeaponGrenadeNapalmProjectile),
    #[serde(rename = "tf_weapon_grenade_normal")]
    TfWeaponGrenadeNormal(TfWeaponGrenadeNormal),
    #[serde(rename = "tf_weapon_grenade_normal_projectile")]
    TfWeaponGrenadeNormalProjectile(TfWeaponGrenadeNormalProjectile),
    #[serde(rename = "tf_weapon_grenade_smoke_bomb")]
    TfWeaponGrenadeSmokeBomb(TfWeaponGrenadeSmokeBomb),
    #[serde(rename = "tf_weapon_grenadelauncher")]
    TfWeaponGrenadelauncher(TfWeaponGrenadelauncher),
    #[serde(rename = "tf_weapon_handgun_scout_primary")]
    TfWeaponHandgunScoutPrimary(TfWeaponHandgunScoutPrimary),
    #[serde(rename = "tf_weapon_handgun_scout_secondary")]
    TfWeaponHandgunScoutSecondary(TfWeaponHandgunScoutSecondary),
    #[serde(rename = "tf_weapon_invis")]
    TfWeaponInvis(TfWeaponInvis),
    #[serde(rename = "tf_weapon_jar")]
    TfWeaponJar(TfWeaponJar),
    #[serde(rename = "tf_weapon_jar_gas")]
    TfWeaponJarGas(TfWeaponJarGas),
    #[serde(rename = "tf_weapon_jar_milk")]
    TfWeaponJarMilk(TfWeaponJarMilk),
    #[serde(rename = "tf_weapon_katana")]
    TfWeaponKatana(TfWeaponKatana),
    #[serde(rename = "tf_weapon_knife")]
    TfWeaponKnife(TfWeaponKnife),
    #[serde(rename = "tf_weapon_laser_pointer")]
    TfWeaponLaserPointer(TfWeaponLaserPointer),
    #[serde(rename = "tf_weapon_lunchbox")]
    TfWeaponLunchbox(TfWeaponLunchbox),
    #[serde(rename = "tf_weapon_lunchbox_drink")]
    TfWeaponLunchboxDrink(TfWeaponLunchboxDrink),
    #[serde(rename = "tf_weapon_mechanical_arm")]
    TfWeaponMechanicalArm(TfWeaponMechanicalArm),
    #[serde(rename = "tf_weapon_medigun")]
    TfWeaponMedigun(TfWeaponMedigun),
    #[serde(rename = "tf_weapon_minigun")]
    TfWeaponMinigun(TfWeaponMinigun),
    #[serde(rename = "tf_weapon_nailgun")]
    TfWeaponNailgun(TfWeaponNailgun),
    #[serde(rename = "tf_weapon_particle_cannon")]
    TfWeaponParticleCannon(TfWeaponParticleCannon),
    #[serde(rename = "tf_weapon_passtime_gun")]
    TfWeaponPasstimeGun(TfWeaponPasstimeGun),
    #[serde(rename = "tf_weapon_pda_engineer_build")]
    TfWeaponPdaEngineerBuild(TfWeaponPdaEngineerBuild),
    #[serde(rename = "tf_weapon_pda_engineer_destroy")]
    TfWeaponPdaEngineerDestroy(TfWeaponPdaEngineerDestroy),
    #[serde(rename = "tf_weapon_pda_spy")]
    TfWeaponPdaSpy(TfWeaponPdaSpy),
    #[serde(rename = "tf_weapon_pipebomblauncher")]
    TfWeaponPipebomblauncher(TfWeaponPipebomblauncher),
    #[serde(rename = "tf_weapon_pistol")]
    TfWeaponPistol(TfWeaponPistol),
    #[serde(rename = "tf_weapon_pistol_scout")]
    TfWeaponPistolScout(TfWeaponPistolScout),
    #[serde(rename = "tf_weapon_raygun")]
    TfWeaponRaygun(TfWeaponRaygun),
    #[serde(rename = "tf_weapon_raygun")]
    TfWeaponRaygun(TfWeaponRaygun),
    #[serde(rename = "tf_weapon_revolver")]
    TfWeaponRevolver(TfWeaponRevolver),
    #[serde(rename = "tf_weapon_robot_arm")]
    TfWeaponRobotArm(TfWeaponRobotArm),
    #[serde(rename = "tf_weapon_rocketlauncher")]
    TfWeaponRocketlauncher(TfWeaponRocketlauncher),
    #[serde(rename = "tf_weapon_rocketlauncher_airstrike")]
    TfWeaponRocketlauncherAirstrike(TfWeaponRocketlauncherAirstrike),
    #[serde(rename = "tf_weapon_rocketlauncher_directhit")]
    TfWeaponRocketlauncherDirecthit(TfWeaponRocketlauncherDirecthit),
    #[serde(rename = "tf_weapon_rocketlauncher_fireball")]
    TfWeaponRocketlauncherFireball(TfWeaponRocketlauncherFireball),
    #[serde(rename = "tf_weapon_rocketpack")]
    TfWeaponRocketpack(TfWeaponRocketpack),
    #[serde(rename = "tf_weapon_sapper")]
    TfWeaponSapper(TfWeaponSapper),
    #[serde(rename = "tf_weapon_shovel")]
    TfWeaponShovel(TfWeaponShovel),
    #[serde(rename = "tf_weapon_slap")]
    TfWeaponSlap(TfWeaponSlap),
    #[serde(rename = "tf_weapon_sniperrifle")]
    TfWeaponSniperrifle(TfWeaponSniperrifle),
    #[serde(rename = "tf_weapon_sniperrifle_classic")]
    TfWeaponSniperrifleClassic(TfWeaponSniperrifleClassic),
    #[serde(rename = "tf_weapon_sniperrifle_decap")]
    TfWeaponSniperrifleDecap(TfWeaponSniperrifleDecap),
    #[serde(rename = "tf_weapon_spellbook")]
    TfWeaponSpellbook(TfWeaponSpellbook),
    #[serde(rename = "tf_weapon_stickbomb")]
    TfWeaponStickbomb(TfWeaponStickbomb),
    #[serde(rename = "tf_weapon_sword")]
    TfWeaponSword(TfWeaponSword),
    #[serde(rename = "tf_weapon_syringegun_medic")]
    TfWeaponSyringegunMedic(TfWeaponSyringegunMedic),
    #[serde(rename = "tf_weapon_throwable")]
    TfWeaponThrowable(TfWeaponThrowable),
    #[serde(rename = "tf_weapon_tranq")]
    TfWeaponTranq(TfWeaponTranq),
    #[serde(rename = "tf_weapon_wrench")]
    TfWeaponWrench(TfWeaponWrench),
    #[serde(rename = "tf_weaponbase_grenade")]
    TfWeaponbaseGrenade(TfWeaponbaseGrenade),
    #[serde(rename = "tf_weaponbase_grenade_proj")]
    TfWeaponbaseGrenadeProj(TfWeaponbaseGrenadeProj),
    #[serde(rename = "tf_weaponbase_melee")]
    TfWeaponbaseMelee(TfWeaponbaseMelee),
    #[serde(rename = "tf_weaponbase_merasmus_grenade")]
    TfWeaponbaseMerasmusGrenade(TfWeaponbaseMerasmusGrenade),
    #[serde(rename = "tf_wearable")]
    TfWearable(TfWearable),
    #[serde(rename = "tf_wearable_campaign_item")]
    TfWearableCampaignItem(TfWearableCampaignItem),
    #[serde(rename = "tf_wearable_demoshield")]
    TfWearableDemoshield(TfWearableDemoshield),
    #[serde(rename = "tf_wearable_levelable_item")]
    TfWearableLevelableItem(TfWearableLevelableItem),
    #[serde(rename = "tf_wearable_razorback")]
    TfWearableRazorback(TfWearableRazorback),
    #[serde(rename = "tf_wearable_robot_arm")]
    TfWearableRobotArm(TfWearableRobotArm),
    #[serde(rename = "tf_wearable_vm")]
    TfWearableVm(TfWearableVm),
    #[serde(rename = "tf_zombie")]
    TfZombie(TfZombie),
    #[serde(rename = "tf_zombie_spawner")]
    TfZombieSpawner(TfZombieSpawner),
    #[serde(rename = "training_annotation")]
    #[serde(borrow)]
    TrainingAnnotation(TrainingAnnotation<'a>),
    #[serde(rename = "training_prop_dynamic")]
    #[serde(borrow)]
    TrainingPropDynamic(TrainingPropDynamic<'a>),
    #[serde(rename = "trigger")]
    #[serde(borrow)]
    Trigger(Trigger<'a>),
    #[serde(rename = "trigger_add_or_remove_tf_player_attributes")]
    #[serde(borrow)]
    TriggerAddOrRemoveTfPlayerAttributes(TriggerAddOrRemoveTfPlayerAttributes<'a>),
    #[serde(rename = "trigger_add_tf_player_condition")]
    #[serde(borrow)]
    TriggerAddTfPlayerCondition(TriggerAddTfPlayerCondition<'a>),
    #[serde(rename = "trigger_apply_impulse")]
    #[serde(borrow)]
    TriggerApplyImpulse(TriggerApplyImpulse<'a>),
    #[serde(rename = "trigger_autosave")]
    #[serde(borrow)]
    TriggerAutosave(TriggerAutosave<'a>),
    #[serde(rename = "trigger_bot_tag")]
    #[serde(borrow)]
    TriggerBotTag(TriggerBotTag<'a>),
    #[serde(rename = "trigger_brush")]
    #[serde(borrow)]
    TriggerBrush(TriggerBrush<'a>),
    #[serde(rename = "trigger_capture_area")]
    #[serde(borrow)]
    TriggerCaptureArea(TriggerCaptureArea<'a>),
    #[serde(rename = "trigger_catapult")]
    #[serde(borrow)]
    TriggerCatapult(TriggerCatapult<'a>),
    #[serde(rename = "trigger_cdaudio")]
    #[serde(borrow)]
    TriggerCdaudio(TriggerCdaudio<'a>),
    #[serde(rename = "trigger_changelevel")]
    #[serde(borrow)]
    TriggerChangelevel(TriggerChangelevel<'a>),
    #[serde(rename = "trigger_gravity")]
    #[serde(borrow)]
    TriggerGravity(TriggerGravity<'a>),
    #[serde(rename = "trigger_hurt")]
    TriggerHurt(TriggerHurt),
    #[serde(rename = "trigger_ignite")]
    #[serde(borrow)]
    TriggerIgnite(TriggerIgnite<'a>),
    #[serde(rename = "trigger_ignite_arrows")]
    #[serde(borrow)]
    TriggerIgniteArrows(TriggerIgniteArrows<'a>),
    #[serde(rename = "trigger_impact")]
    TriggerImpact(TriggerImpact),
    #[serde(rename = "trigger_look")]
    TriggerLook(TriggerLook),
    #[serde(rename = "trigger_multiple")]
    #[serde(borrow)]
    TriggerMultiple(TriggerMultiple<'a>),
    #[serde(rename = "trigger_once")]
    TriggerOnce(TriggerOnce),
    #[serde(rename = "trigger_particle")]
    #[serde(borrow)]
    TriggerParticle(TriggerParticle<'a>),
    #[serde(rename = "trigger_passtime_ball")]
    #[serde(borrow)]
    TriggerPasstimeBall(TriggerPasstimeBall<'a>),
    #[serde(rename = "trigger_physics_trap")]
    TriggerPhysicsTrap(TriggerPhysicsTrap),
    #[serde(rename = "trigger_player_respawn_override")]
    #[serde(borrow)]
    TriggerPlayerRespawnOverride(TriggerPlayerRespawnOverride<'a>),
    #[serde(rename = "trigger_playermovement")]
    #[serde(borrow)]
    TriggerPlayermovement(TriggerPlayermovement<'a>),
    #[serde(rename = "trigger_portal")]
    #[serde(borrow)]
    TriggerPortal(TriggerPortal<'a>),
    #[serde(rename = "trigger_proximity")]
    #[serde(borrow)]
    TriggerProximity(TriggerProximity<'a>),
    #[serde(rename = "trigger_push")]
    #[serde(borrow)]
    TriggerPush(TriggerPush<'a>),
    #[serde(rename = "trigger_rd_vault_trigger")]
    #[serde(borrow)]
    TriggerRdVaultTrigger(TriggerRdVaultTrigger<'a>),
    #[serde(rename = "trigger_remove")]
    #[serde(borrow)]
    TriggerRemove(TriggerRemove<'a>),
    #[serde(rename = "trigger_remove_tf_player_condition")]
    #[serde(borrow)]
    TriggerRemoveTfPlayerCondition(TriggerRemoveTfPlayerCondition<'a>),
    #[serde(rename = "trigger_rpgfire")]
    TriggerRpgfire(TriggerRpgfire),
    #[serde(rename = "trigger_serverragdoll")]
    #[serde(borrow)]
    TriggerServerragdoll(TriggerServerragdoll<'a>),
    #[serde(rename = "trigger_soundscape")]
    #[serde(borrow)]
    TriggerSoundscape(TriggerSoundscape<'a>),
    #[serde(rename = "trigger_stun")]
    #[serde(borrow)]
    TriggerStun(TriggerStun<'a>),
    #[serde(rename = "trigger_super_armor")]
    TriggerSuperArmor(TriggerSuperArmor),
    #[serde(rename = "trigger_teleport")]
    #[serde(borrow)]
    TriggerTeleport(TriggerTeleport<'a>),
    #[serde(rename = "trigger_teleport_relative")]
    #[serde(borrow)]
    TriggerTeleportRelative(TriggerTeleportRelative<'a>),
    #[serde(rename = "trigger_timer_door")]
    #[serde(borrow)]
    TriggerTimerDoor(TriggerTimerDoor<'a>),
    #[serde(rename = "trigger_togglesave")]
    #[serde(borrow)]
    TriggerTogglesave(TriggerTogglesave<'a>),
    #[serde(rename = "trigger_transition")]
    TriggerTransition(TriggerTransition),
    #[serde(rename = "trigger_vehicle_cargo")]
    #[serde(borrow)]
    TriggerVehicleCargo(TriggerVehicleCargo<'a>),
    #[serde(rename = "trigger_vphysics_motion")]
    #[serde(borrow)]
    TriggerVphysicsMotion(TriggerVphysicsMotion<'a>),
    #[serde(rename = "trigger_waterydeath")]
    #[serde(borrow)]
    TriggerWaterydeath(TriggerWaterydeath<'a>),
    #[serde(rename = "trigger_weapon_dissolve")]
    #[serde(borrow)]
    TriggerWeaponDissolve(TriggerWeaponDissolve<'a>),
    #[serde(rename = "trigger_weapon_strip")]
    TriggerWeaponStrip(TriggerWeaponStrip),
    #[serde(rename = "trigger_wind")]
    #[serde(borrow)]
    TriggerWind(TriggerWind<'a>),
    #[serde(rename = "tripwire_hook")]
    #[serde(borrow)]
    TripwireHook(TripwireHook<'a>),
    #[serde(rename = "vehicle_viewcontroller")]
    VehicleViewcontroller(VehicleViewcontroller),
    #[serde(rename = "vgui_screen")]
    #[serde(borrow)]
    VguiScreen(VguiScreen<'a>),
    #[serde(rename = "vgui_screen_team")]
    #[serde(borrow)]
    VguiScreenTeam(VguiScreenTeam<'a>),
    #[serde(rename = "vgui_slideshow_display")]
    #[serde(borrow)]
    VguiSlideshowDisplay(VguiSlideshowDisplay<'a>),
    #[serde(rename = "viewangleanim")]
    Viewangleanim(Viewangleanim),
    #[serde(rename = "viewmodel")]
    #[serde(borrow)]
    Viewmodel(Viewmodel<'a>),
    #[serde(rename = "vort_charge_token")]
    #[serde(borrow)]
    VortChargeToken(VortChargeToken<'a>),
    #[serde(rename = "vort_effect_dispel")]
    #[serde(borrow)]
    VortEffectDispel(VortEffectDispel<'a>),
    #[serde(rename = "vortex_controller")]
    #[serde(borrow)]
    VortexController(VortexController<'a>),
    #[serde(rename = "vote_controller")]
    #[serde(borrow)]
    VoteController(VoteController<'a>),
    #[serde(rename = "water_lod_control")]
    #[serde(borrow)]
    WaterLodControl(WaterLodControl<'a>),
    #[serde(rename = "waterbullet")]
    #[serde(borrow)]
    Waterbullet(Waterbullet<'a>),
    #[serde(rename = "weapon_357")]
    Weapon357(Weapon357),
    #[serde(rename = "weapon_357")]
    Weapon357(Weapon357),
    #[serde(rename = "weapon_alyxgun")]
    WeaponAlyxgun(WeaponAlyxgun),
    #[serde(rename = "weapon_annabelle")]
    WeaponAnnabelle(WeaponAnnabelle),
    #[serde(rename = "weapon_ar1")]
    WeaponAr1(WeaponAr1),
    #[serde(rename = "weapon_ar2")]
    WeaponAr2(WeaponAr2),
    #[serde(rename = "weapon_ar2")]
    WeaponAr2(WeaponAr2),
    #[serde(rename = "weapon_brickbat")]
    WeaponBrickbat(WeaponBrickbat),
    #[serde(rename = "weapon_bugbait")]
    WeaponBugbait(WeaponBugbait),
    #[serde(rename = "weapon_cguard")]
    WeaponCguard(WeaponCguard),
    #[serde(rename = "weapon_citizenpackage")]
    WeaponCitizenpackage(WeaponCitizenpackage),
    #[serde(rename = "weapon_citizensuitcase")]
    WeaponCitizensuitcase(WeaponCitizensuitcase),
    #[serde(rename = "weapon_crossbow")]
    WeaponCrossbow(WeaponCrossbow),
    #[serde(rename = "weapon_crossbow")]
    WeaponCrossbow(WeaponCrossbow),
    #[serde(rename = "weapon_crowbar")]
    WeaponCrowbar(WeaponCrowbar),
    #[serde(rename = "weapon_crowbar")]
    WeaponCrowbar(WeaponCrowbar),
    #[serde(rename = "weapon_crowbar")]
    WeaponCrowbar(WeaponCrowbar),
    #[serde(rename = "weapon_cubemap")]
    WeaponCubemap(WeaponCubemap),
    #[serde(rename = "weapon_extinguisher")]
    WeaponExtinguisher(WeaponExtinguisher),
    #[serde(rename = "weapon_flaregun")]
    WeaponFlaregun(WeaponFlaregun),
    #[serde(rename = "weapon_frag")]
    WeaponFrag(WeaponFrag),
    #[serde(rename = "weapon_frag")]
    WeaponFrag(WeaponFrag),
    #[serde(rename = "weapon_grenade")]
    WeaponGrenade(WeaponGrenade),
    #[serde(rename = "weapon_hl2mp_base")]
    WeaponHl2mpBase(WeaponHl2mpBase),
    #[serde(rename = "weapon_hopwire")]
    WeaponHopwire(WeaponHopwire),
    #[serde(rename = "weapon_ifm_base")]
    WeaponIfmBase(WeaponIfmBase),
    #[serde(rename = "weapon_ifm_base_camera")]
    WeaponIfmBaseCamera(WeaponIfmBaseCamera),
    #[serde(rename = "weapon_ifm_steadycam")]
    WeaponIfmSteadycam(WeaponIfmSteadycam),
    #[serde(rename = "weapon_immolator")]
    WeaponImmolator(WeaponImmolator),
    #[serde(rename = "weapon_manhack")]
    WeaponManhack(WeaponManhack),
    #[serde(rename = "weapon_molotov")]
    WeaponMolotov(WeaponMolotov),
    #[serde(rename = "weapon_mp5")]
    WeaponMp5(WeaponMp5),
    #[serde(rename = "weapon_oldmanharpoon")]
    WeaponOldmanharpoon(WeaponOldmanharpoon),
    #[serde(rename = "weapon_physcannon")]
    WeaponPhyscannon(WeaponPhyscannon),
    #[serde(rename = "weapon_physcannon")]
    WeaponPhyscannon(WeaponPhyscannon),
    #[serde(rename = "weapon_physgun")]
    WeaponPhysgun(WeaponPhysgun),
    #[serde(rename = "weapon_pistol")]
    WeaponPistol(WeaponPistol),
    #[serde(rename = "weapon_pistol")]
    WeaponPistol(WeaponPistol),
    #[serde(rename = "weapon_pistol")]
    WeaponPistol(WeaponPistol),
    #[serde(rename = "weapon_proto1")]
    WeaponProto1(WeaponProto1),
    #[serde(rename = "weapon_rpg")]
    WeaponRpg(WeaponRpg),
    #[serde(rename = "weapon_rpg")]
    WeaponRpg(WeaponRpg),
    #[serde(rename = "weapon_sdk_base")]
    WeaponSdkBase(WeaponSdkBase),
    #[serde(rename = "weapon_shotgun")]
    WeaponShotgun(WeaponShotgun),
    #[serde(rename = "weapon_shotgun")]
    WeaponShotgun(WeaponShotgun),
    #[serde(rename = "weapon_shotgun")]
    WeaponShotgun(WeaponShotgun),
    #[serde(rename = "weapon_slam")]
    WeaponSlam(WeaponSlam),
    #[serde(rename = "weapon_slam")]
    WeaponSlam(WeaponSlam),
    #[serde(rename = "weapon_smg1")]
    WeaponSmg1(WeaponSmg1),
    #[serde(rename = "weapon_smg1")]
    WeaponSmg1(WeaponSmg1),
    #[serde(rename = "weapon_smg2")]
    WeaponSmg2(WeaponSmg2),
    #[serde(rename = "weapon_sniperrifle")]
    WeaponSniperrifle(WeaponSniperrifle),
    #[serde(rename = "weapon_striderbuster")]
    #[serde(borrow)]
    WeaponStriderbuster(WeaponStriderbuster<'a>),
    #[serde(rename = "weapon_stunstick")]
    WeaponStunstick(WeaponStunstick),
    #[serde(rename = "weapon_stunstick")]
    WeaponStunstick(WeaponStunstick),
    #[serde(rename = "weapon_thumper")]
    WeaponThumper(WeaponThumper),
    #[serde(rename = "weapon_thumper")]
    WeaponThumper(WeaponThumper),
    #[serde(rename = "weapon_tripwire")]
    WeaponTripwire(WeaponTripwire),
    #[serde(rename = "wearable_item")]
    WearableItem(WearableItem),
    #[serde(rename = "wheel_of_doom")]
    #[serde(borrow)]
    WheelOfDoom(WheelOfDoom<'a>),
    #[serde(rename = "wheel_of_doom_spiral")]
    #[serde(borrow)]
    WheelOfDoomSpiral(WheelOfDoomSpiral<'a>),
    #[serde(rename = "window_pane")]
    #[serde(borrow)]
    WindowPane(WindowPane<'a>),
    #[serde(rename = "world_items")]
    #[serde(borrow)]
    WorldItems(WorldItems<'a>),
    #[serde(rename = "worldspawn")]
    #[serde(borrow)]
    Worldspawn(Worldspawn<'a>),
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiAllyManager<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxallies: i32,
    pub maxmedics: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiAllySpeechManager {}
#[derive(Debug, Clone, Deserialize)]
pub struct AiBattleLine<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Active: bool,
    pub Actor: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub Strict: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiChangehintgroup<'a> {
    pub NewHintGroup: &'a str,
    pub Radius: f32,
    pub SearchName: &'a str,
    pub SearchType: i32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiChangetarget<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub m_iszNewTarget: &'a str,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiCitizenResponseSystem<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiGoalActbusy<'a> {
    pub Actor: &'a str,
    pub BaseConceptModifiers: &'a str,
    pub Goal: &'a str,
    pub SafeZone: &'a str,
    pub SearchType: i32,
    pub SeeEntity: &'a str,
    pub SeeEntityTimeout: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartActive: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowteleport: bool,
    pub busysearchrange: f32,
    pub r#type: i32,
    pub sightmethod: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub visibleonly: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiGoalActbusyQueue<'a> {
    pub SafeZone: &'a str,
    pub SeeEntity: &'a str,
    pub SeeEntityTimeout: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowteleport: bool,
    pub busysearchrange: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub mustreachfront: bool,
    pub node01: &'a str,
    pub node02: &'a str,
    pub node03: &'a str,
    pub node04: &'a str,
    pub node05: &'a str,
    pub node06: &'a str,
    pub node07: &'a str,
    pub node08: &'a str,
    pub node09: &'a str,
    pub node10: &'a str,
    pub node11: &'a str,
    pub node12: &'a str,
    pub node13: &'a str,
    pub node14: &'a str,
    pub node15: &'a str,
    pub node16: &'a str,
    pub node17: &'a str,
    pub node18: &'a str,
    pub node19: &'a str,
    pub node20: &'a str,
    pub node_exit: &'a str,
    pub r#type: i32,
    pub sightmethod: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub visibleonly: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiGoalAssault<'a> {
    pub Actor: &'a str,
    pub AssaultCue: i32,
    pub BaseConceptModifiers: &'a str,
    pub Goal: &'a str,
    pub RallySelectMethod: i32,
    pub SearchType: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartActive: bool,
    pub rallypoint: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiGoalFollow<'a> {
    pub Actor: &'a str,
    pub BaseConceptModifiers: &'a str,
    pub Formation: i32,
    pub Goal: &'a str,
    pub SearchType: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartActive: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiGoalInjuredFollow {
    pub Formation: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiGoalLead<'a> {
    pub Actor: &'a str,
    pub ArrivalConceptModifier: &'a str,
    pub AttractPlayerConceptModifier: &'a str,
    pub BaseConceptModifiers: &'a str,
    pub ComingBackConceptModifier: &'a str,
    pub ComingBackWaitForSpeak: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontSpeakStart: bool,
    pub FailureConceptModifier: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub GagLeader: bool,
    pub Goal: &'a str,
    pub LeadDistance: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub LeadDuringCombat: bool,
    pub PostArrivalConceptModifier: &'a str,
    pub Retrieve: i32,
    pub RetrieveConceptModifier: &'a str,
    pub RetrieveDistance: f32,
    pub RetrieveWaitForSpeak: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub Run: bool,
    pub SearchType: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartActive: bool,
    pub StartConceptModifier: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StopScenes: bool,
    pub SuccessConceptModifier: &'a str,
    pub SuccessDistance: f32,
    pub WaitDistance: f32,
    pub WaitOverConceptModifier: &'a str,
    pub WaitPointName: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiGoalLeadWeapon<'a> {
    pub ArrivalConceptModifier: &'a str,
    pub AttractPlayerConceptModifier: &'a str,
    pub ComingBackConceptModifier: &'a str,
    pub ComingBackWaitForSpeak: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontSpeakStart: bool,
    pub FailureConceptModifier: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub GagLeader: bool,
    pub LeadDistance: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub LeadDuringCombat: bool,
    pub MissingWeaponConceptModifier: &'a str,
    pub PostArrivalConceptModifier: &'a str,
    pub Retrieve: i32,
    pub RetrieveConceptModifier: &'a str,
    pub RetrieveDistance: f32,
    pub RetrieveWaitForSpeak: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub Run: bool,
    pub StartConceptModifier: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StopScenes: bool,
    pub SuccessConceptModifier: &'a str,
    pub SuccessDistance: f32,
    pub WaitDistance: f32,
    pub WaitOverConceptModifier: &'a str,
    pub WaitPointName: &'a str,
    pub WeaponName: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiGoalOperator<'a> {
    pub Actor: &'a str,
    pub BaseConceptModifiers: &'a str,
    pub Goal: &'a str,
    pub SearchType: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartActive: bool,
    pub contexttarget: &'a str,
    pub moveto: i32,
    pub state: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiGoalPolice<'a> {
    pub PoliceRadius: f32,
    pub PoliceTarget: &'a str,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiGoalStandoff<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AbandonIfEnemyHides: bool,
    pub Actor: &'a str,
    pub Aggressiveness: i32,
    pub BaseConceptModifiers: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub CustomCoverOnReload: bool,
    pub CustomMaxShots: i32,
    pub CustomMaxTimeShots: f32,
    pub CustomMinShots: i32,
    pub CustomMinTimeShots: f32,
    pub CustomOddsCover: i32,
    pub Goal: &'a str,
    pub HintGroupChangeReaction: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub PlayerBattleline: bool,
    pub SearchType: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartActive: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StayAtCover: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiHint {
    pub nodeFOV: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiLooktarget {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub context: i32,
    pub maxdist: f32,
    pub priority: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiNetwork {}
#[derive(Debug, Clone, Deserialize)]
pub struct AiNetworkBuildHelper {}
#[derive(Debug, Clone, Deserialize)]
pub struct AiNpcEventresponsesystem<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiRelationship<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartActive: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub disposition: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub radius: f32,
    pub rank: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub reciprocal: bool,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub subject: &'a str,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiScriptConditions<'a> {
    pub Actor: &'a str,
    pub ActorInPVS: i32,
    pub ActorInVehicle: i32,
    pub ActorSeePlayer: i32,
    pub ActorSeeTarget: i32,
    pub ActorTargetProximity: f32,
    pub MaxTimeout: f32,
    pub MaximumState: i32,
    pub MinTimeout: f32,
    pub MinimumState: i32,
    pub PlayerActorFOV: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub PlayerActorFOVTrueCone: bool,
    pub PlayerActorLOS: i32,
    pub PlayerActorProximity: f32,
    pub PlayerBlockingActor: i32,
    pub PlayerInVehicle: i32,
    pub PlayerTargetFOV: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub PlayerTargetFOVTrueCone: bool,
    pub PlayerTargetLOS: i32,
    pub PlayerTargetProximity: f32,
    pub RequiredTime: f32,
    pub ScriptStatus: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiSound<'a> {
    pub duration: f32,
    pub locationproxy: &'a str,
    pub soundcontext: i32,
    pub soundtype: i32,
    pub volume: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiSpeechfilter<'a> {
    pub IdleModifier: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub NeverSayHello: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub subject: &'a str,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AiscriptedSchedule<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub forcestate: i32,
    pub goalent: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub graball: bool,
    pub interruptability: i32,
    pub m_flRadius: f32,
    pub m_iszEntity: &'a str,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub schedule: i32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Aitesthull<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AmbientGeneric<'a> {
    pub SourceEntityName: &'a str,
    pub cspinup: i32,
    pub fadein: i32,
    pub fadeinsecs: i32,
    pub fadeout: i32,
    pub fadeoutsecs: i32,
    pub lfomodpitch: i32,
    pub lfomodvol: i32,
    pub lforate: i32,
    pub lfotype: i32,
    pub pitch: i32,
    pub pitchstart: i32,
    pub preset: i32,
    pub radius: f32,
    pub spindown: i32,
    pub spinup: i32,
    pub volstart: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ApcMissile {}
#[derive(Debug, Clone, Deserialize)]
pub struct ApcMissile {}
#[derive(Debug, Clone, Deserialize)]
pub struct Ar2explosion {}
#[derive(Debug, Clone, Deserialize)]
pub struct ArcherProxy<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AssaultAssaultpoint<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowdiversion: bool,
    pub allowdiversionradius: f32,
    pub assaultgroup: &'a str,
    pub assaulttimeout: f32,
    pub assaulttolerance: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub clearoncontact: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub forcecrouch: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nevertimeout: bool,
    pub nextassaultpoint: &'a str,
    pub strict: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub urgent: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct AssaultRallypoint<'a> {
    pub assaultdelay: f32,
    pub assaultpoint: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub forcecrouch: bool,
    pub priority: i32,
    pub rallysequence: &'a str,
    pub strict: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub urgent: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Ballplayertoucher<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BaseBoss<'a> {
    pub health: i32,
    pub model: &'a str,
    pub speed: f32,
    pub start_disabled: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BaseBoss {}
#[derive(Debug, Clone, Deserialize)]
pub struct BaseZombie {}
#[derive(Debug, Clone, Deserialize)]
pub struct Basehl2mpcombatweapon {}
#[derive(Debug, Clone, Deserialize)]
pub struct Basehlcombatweapon {}
#[derive(Debug, Clone, Deserialize)]
pub struct Baseprojectile {
    pub GlowProxySize: f32,
    pub HDRColorScale: f32,
    pub frame: f32,
    pub framerate: f32,
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Beam<'a> {
    pub HDRColorScale: f32,
    pub angles: Angles,
    pub damage: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub dissolvetype: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BlobElement<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Bodyque<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BossAlpha {}
#[derive(Debug, Clone, Deserialize)]
pub struct BotActionPoint<'a> {
    pub command: &'a str,
    pub desired_distance: f32,
    pub next_action_point: &'a str,
    pub stay_time: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotActionPoint<'a> {
    pub command: &'a str,
    pub desired_distance: f32,
    pub next_action_point: &'a str,
    pub stay_time: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotBoss {}
#[derive(Debug, Clone, Deserialize)]
pub struct BotBossMiniNuker {}
#[derive(Debug, Clone, Deserialize)]
pub struct BotBossMiniRockets {}
#[derive(Debug, Clone, Deserialize)]
pub struct BotController<'a> {
    pub bot_class: i32,
    pub bot_name: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotGenerator<'a> {
    pub actionOnDeath: i32,
    pub action_point: &'a str,
    pub count: i32,
    pub difficulty: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableDodge: bool,
    pub initial_command: &'a str,
    pub interval: f32,
    pub maxActive: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub retainBuildings: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub spawnOnlyWhenTriggered: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub suppressFire: bool,
    pub team: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub useTeamSpawnPoint: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotGenerator<'a> {
    pub actionOnDeath: i32,
    pub action_point: &'a str,
    pub class: &'a str,
    pub count: i32,
    pub difficulty: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableDodge: bool,
    pub initial_command: &'a str,
    pub interval: f32,
    pub maxActive: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub retainBuildings: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub spawnOnlyWhenTriggered: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub suppressFire: bool,
    pub team: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub useTeamSpawnPoint: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotHintEngineerNest {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotHintSentrygun {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub sticky: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotHintTeleporterExit {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotNpcArcher {}
#[derive(Debug, Clone, Deserialize)]
pub struct BotNpcDecoy {}
#[derive(Debug, Clone, Deserialize)]
pub struct BotNpcMinion {}
#[derive(Debug, Clone, Deserialize)]
pub struct BotProxy<'a> {
    pub action_point: &'a str,
    pub bot_name: &'a str,
    pub respawn_interval: f32,
    pub spawn_on_start: &'a str,
    pub team: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotProxy<'a> {
    pub action_point: &'a str,
    pub bot_name: &'a str,
    pub class: &'a str,
    pub respawn_interval: f32,
    pub spawn_on_start: &'a str,
    pub team: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BotRoster<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowClassChanges: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowDemoman: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowEngineer: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowHeavy: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowMedic: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowPyro: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowScout: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowSniper: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowSoldier: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowSpy: bool,
    pub team: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct BounceBomb<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Bounce: bool,
    pub ExplosionDelay: f32,
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub LockSilently: bool,
    pub Modification: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisarmed: bool,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Brickbat {}
#[derive(Debug, Clone, Deserialize)]
pub struct BullseyeStriderFocus<'a> {
    pub additionalequipment: &'a str,
    pub autoaimradius: f32,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub minangle: f32,
    pub mindist: f32,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ClientRagdoll {}
#[derive(Debug, Clone, Deserialize)]
pub struct ColorCorrection<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub enabled: bool,
    pub fadeInDuration: f32,
    pub fadeOutDuration: f32,
    pub filename: &'a str,
    pub maxfalloff: f32,
    pub maxs: Vector,
    pub maxweight: f32,
    pub minfalloff: f32,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ColorCorrectionVolume<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub enabled: bool,
    pub fadeDuration: f32,
    pub filtername: &'a str,
    pub maxweight: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CombineArmorPiece<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CombineBouncemine<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Bounce: bool,
    pub ExplosionDelay: f32,
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub LockSilently: bool,
    pub Modification: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisarmed: bool,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CombineMine<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Bounce: bool,
    pub ExplosionDelay: f32,
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub LockSilently: bool,
    pub Modification: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisarmed: bool,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CommentaryAuto<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Concussiveblast<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CraneTip<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CrossbowBolt<'a> {
    pub Relationship: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CrossbowBolt<'a> {
    pub Relationship: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Cycler {}
#[derive(Debug, Clone, Deserialize)]
pub struct CyclerActor<'a> {
    pub Sentence: &'a str,
    pub hull_name: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CyclerBlender<'a> {
    pub blendsequence: &'a str,
    pub blendspeed: i32,
    pub highboundary: i32,
    pub lowboundary: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CyclerFlex<'a> {
    pub Sentence: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct CyclerWeapon {}
#[derive(Debug, Clone, Deserialize)]
pub struct CyclerWreckage<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DelayedUse {}
#[derive(Debug, Clone, Deserialize)]
pub struct DispenserTouchTrigger<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct DynamicProp<'a> {
    pub BreakModelMessage: &'a str,
    pub DefaultAnim: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DisableBoneFollowers: bool,
    pub ExplodeDamage: f32,
    pub ExplodeRadius: f32,
    pub MaxAnimTime: f32,
    pub MinAnimTime: f32,
    pub PerformanceMode: i32,
    pub PressureDelay: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub RandomAnimation: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub minhealthdmg: i32,
    pub puntsound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EntWateryLeech<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EntityBird<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EntityBlocker<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EntityCarrier {}
#[derive(Debug, Clone, Deserialize)]
pub struct EntityCroc<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EntityMedigunShield<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EntityName {}
#[derive(Debug, Clone, Deserialize)]
pub struct EntityReviveMarker<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EntityRocket {}
#[derive(Debug, Clone, Deserialize)]
pub struct EntitySaucer {}
#[derive(Debug, Clone, Deserialize)]
pub struct EntitySign {}
#[derive(Debug, Clone, Deserialize)]
pub struct EntitySoldierStatue<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EntitySpawnManager<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub drop_to_ground: bool,
    pub entity_count: i32,
    pub entity_name: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub random_rotation: bool,
    pub respawn_time: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EntitySpawnPoint<'a> {
    pub spawn_manager_name: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Entityflame<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub lifetime: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Entityname {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvAlyxemp<'a> {
    pub EndTargetName: &'a str,
    pub Type: i32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvAr2explosion<'a> {
    pub material: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvAssassinsmoke {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvBeam<'a> {
    pub BoltWidth: f32,
    pub HDRColorScale: f32,
    pub LightningEnd: &'a str,
    pub LightningStart: &'a str,
    pub NoiseAmplitude: f32,
    pub Radius: f32,
    pub StrikeTime: f32,
    pub TextureScroll: i32,
    pub TouchType: i32,
    pub damage: f32,
    pub decalname: &'a str,
    pub dissolvetype: i32,
    pub filtername: &'a str,
    pub framestart: i32,
    pub life: f32,
    pub texture: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvBeverage<'a> {
    pub angles: Angles,
    pub beveragetype: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvBlood {
    pub amount: f32,
    pub color: i32,
    pub spraydir: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvBubbles<'a> {
    pub angles: Angles,
    pub current: f32,
    pub density: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub frequency: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvCitadelEnergyCore<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub scale: f32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvCredits {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvDebughistory<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvDetailController<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvDustpuff {
    pub color: Color,
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvDusttrail {
    pub color: Color,
    pub endsize: f32,
    pub lifetime: f32,
    pub maxdirectedspeed: f32,
    pub maxspeed: f32,
    pub mindirectedspeed: f32,
    pub minspeed: f32,
    pub opacity: f32,
    pub spawnradius: f32,
    pub spawnrate: f32,
    pub startsize: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvEffectscript<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub scriptfile: &'a str,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvEmbers {
    pub density: i32,
    pub lifetime: i32,
    pub speed: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvEntityDissolver<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub dissolvetype: i32,
    pub magnitude: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvEntityIgniter<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub lifetime: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvEntityMaker<'a> {
    pub EntityTemplate: &'a str,
    pub PostSpawnDirection: Vector,
    pub PostSpawnDirectionVariance: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub PostSpawnInheritAngles: bool,
    pub PostSpawnSpeed: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvExplosion<'a> {
    pub DamageForce: f32,
    pub fireballsprite: &'a str,
    pub iMagnitude: i32,
    pub iRadiusOverride: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvExtinguisherjet<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub enabled: bool,
    pub length: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub radius: i32,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub size: i32,
    pub strength: f32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFade {
    pub duration: f32,
    pub holdtime: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFire<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    pub damagescale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub fireattack: f32,
    pub firesize: f32,
    pub firetype: i32,
    pub ignitionpoint: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFireTrail {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFiresensor<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub fireradius: f32,
    pub heatlevel: f32,
    pub heattime: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFiresource<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub firedamage: f32,
    pub fireradius: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFlare<'a> {
    pub Relationship: &'a str,
    pub duration: f32,
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFogController<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub farz: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub fogblend: bool,
    pub fogcolor: Color,
    pub fogcolor2: Color,
    pub fogdir: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub fogenable: bool,
    pub fogend: f32,
    pub foglerptime: f32,
    pub fogmaxdensity: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub fogradial: bool,
    pub fogstart: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub use_angles: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFunnel<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvGlobal<'a> {
    pub counter: i32,
    pub globalstate: &'a str,
    pub initialstate: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvGlow {
    pub GlowProxySize: f32,
    pub HDRColorScale: f32,
    pub frame: f32,
    pub framerate: f32,
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvGunfire<'a> {
    pub bias: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub collisions: bool,
    pub maxburstsize: i32,
    pub minburstsize: i32,
    pub rateoffire: f32,
    pub shootsound: &'a str,
    pub spread: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
    pub tracertype: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvHeadcrabcanister<'a> {
    pub Damage: f32,
    pub DamageRadius: f32,
    pub HeadcrabCount: i32,
    pub HeadcrabType: i32,
    pub LaunchPositionName: &'a str,
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub MaxSkyboxRefireTime: f32,
    pub MinSkyboxRefireTime: f32,
    pub SkyboxCannisterCount: i32,
    pub SmokeLifetime: f32,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvHudhint<'a> {
    pub message: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvLaser<'a> {
    pub EndSprite: &'a str,
    pub HDRColorScale: f32,
    pub LaserTarget: &'a str,
    pub NoiseAmplitude: i32,
    pub TextureScroll: i32,
    pub damage: f32,
    pub dissolvetype: i32,
    pub framestart: f32,
    pub texture: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvLaserdot {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvLaserdot {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvLaserdot {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvLightglow<'a> {
    pub GlowProxySize: f32,
    pub HDRColorScale: f32,
    pub HorizontalGlowSize: i32,
    pub MaxDist: i32,
    pub MinDist: i32,
    pub OuterMaxDist: i32,
    pub VerticalGlowSize: i32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvMessage<'a> {
    pub message: &'a str,
    pub messageattenuation: i32,
    pub messagevolume: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvMicrophone<'a> {
    pub ListenFilter: &'a str,
    pub MaxRange: f32,
    pub Sensitivity: f32,
    pub SmoothFactor: f32,
    pub SoundMask: i32,
    pub SpeakerName: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub speaker_dsp_preset: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvMovieexplosion {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvMuzzleflash<'a> {
    pub parentattachment: &'a str,
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvParticlePerformanceMonitor {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvParticleTrail {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvParticlefire {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvParticlelight<'a> {
    pub Color: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub Directional: bool,
    pub Intensity: f32,
    pub PSName: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvParticlescript<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvParticlesmokegrenade {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvPhysexplosion<'a> {
    pub inner_radius: f32,
    pub magnitude: f32,
    pub radius: f32,
    pub targetentityname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvPhysimpact<'a> {
    pub directionentityname: &'a str,
    pub distance: f32,
    pub magnitude: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvPhyswire {
    pub Density: i32,
    pub frequency: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvPlayerSurfaceTrigger {
    pub gamematerial: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvProjectedtexture {
    pub ambient: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub cameraspace: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub enableshadows: bool,
    pub farz: f32,
    pub lightfov: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub lightonlytarget: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub lightworld: bool,
    pub nearz: f32,
    pub shadowquality: i32,
    pub textureframe: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvQuadraticbeam {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvRagdollBoogie<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvRockettrail {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvRotorshooter<'a> {
    pub gibgravityscale: f32,
    pub massoverride: f32,
    pub rotortime: f32,
    pub rotortimevariance: f32,
    pub scale: f32,
    pub shootmodel: &'a str,
    pub shootsounds: i32,
    pub skin: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvRotorwashEmitter<'a> {
    pub altitude: f32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvScreeneffect {
    pub r#type: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvScreenoverlay<'a> {
    pub OverlayName1: &'a str,
    pub OverlayName10: &'a str,
    pub OverlayName2: &'a str,
    pub OverlayName3: &'a str,
    pub OverlayName4: &'a str,
    pub OverlayName5: &'a str,
    pub OverlayName6: &'a str,
    pub OverlayName7: &'a str,
    pub OverlayName8: &'a str,
    pub OverlayName9: &'a str,
    pub OverlayTime1: f32,
    pub OverlayTime10: f32,
    pub OverlayTime2: f32,
    pub OverlayTime3: f32,
    pub OverlayTime4: f32,
    pub OverlayTime5: f32,
    pub OverlayTime6: f32,
    pub OverlayTime7: f32,
    pub OverlayTime8: f32,
    pub OverlayTime9: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvShake {
    pub amplitude: f32,
    pub duration: f32,
    pub frequency: f32,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvShooter<'a> {
    pub LightingOrigin: &'a str,
    pub Simulation: i32,
    pub delay: f32,
    pub gibangles: Vector,
    pub gibanglevelocity: f32,
    pub gibgravityscale: f32,
    pub m_flGibLife: f32,
    pub m_flVariance: f32,
    pub m_flVelocity: f32,
    pub m_iGibs: i32,
    pub massoverride: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nogibshadows: bool,
    pub scale: f32,
    pub shootmodel: &'a str,
    pub shootsounds: i32,
    pub skin: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSmokestack {
    pub BaseSpread: f32,
    pub EndSize: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub InitialState: bool,
    pub Roll: f32,
    pub StartSize: f32,
    pub Twist: f32,
    pub WindAngle: i32,
    pub WindSpeed: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSmoketrail {
    pub endcolor: Color,
    pub endsize: f32,
    pub lifetime: f32,
    pub maxdirectedspeed: f32,
    pub maxspeed: f32,
    pub mindirectedspeed: f32,
    pub minspeed: f32,
    pub opacity: f32,
    pub spawnradius: f32,
    pub spawnrate: f32,
    pub startcolor: Color,
    pub startsize: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSniperdot<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscape<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub position0: &'a str,
    pub position1: &'a str,
    pub position2: &'a str,
    pub position3: &'a str,
    pub position4: &'a str,
    pub position5: &'a str,
    pub position6: &'a str,
    pub position7: &'a str,
    pub radius: f32,
    pub soundscape: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscapeProxy<'a> {
    pub MainSoundscapeName: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub position0: &'a str,
    pub position1: &'a str,
    pub position2: &'a str,
    pub position3: &'a str,
    pub position4: &'a str,
    pub position5: &'a str,
    pub position6: &'a str,
    pub position7: &'a str,
    pub radius: f32,
    pub soundscape: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscapeTriggerable<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub position0: &'a str,
    pub position1: &'a str,
    pub position2: &'a str,
    pub position3: &'a str,
    pub position4: &'a str,
    pub position5: &'a str,
    pub position6: &'a str,
    pub position7: &'a str,
    pub radius: f32,
    pub soundscape: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpark {
    pub Magnitude: i32,
    pub MaxDelay: f32,
    pub TrailLength: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSparkler<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub scale: f32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpeaker<'a> {
    pub concept: &'a str,
    pub delaymax: f32,
    pub delaymin: f32,
    pub rulescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSplash {
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSporeexplosion {
    pub spawnrate: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSporetrail {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSprite {
    pub GlowProxySize: f32,
    pub HDRColorScale: f32,
    pub frame: f32,
    pub framerate: f32,
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpriteOriented {
    pub GlowProxySize: f32,
    pub HDRColorScale: f32,
    pub frame: f32,
    pub framerate: f32,
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpritetrail<'a> {
    pub GlowProxySize: f32,
    pub HDRColorScale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub animate: bool,
    pub endwidth: f32,
    pub frame: f32,
    pub framerate: f32,
    pub lifetime: f32,
    pub scale: f32,
    pub spritename: &'a str,
    pub startwidth: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvStarfield<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSteam {
    pub EndSize: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub InitialState: bool,
    pub RollSpeed: f32,
    pub StartSize: f32,
    pub Type: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSteamjet {
    pub EndSize: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub InitialState: bool,
    pub RollSpeed: f32,
    pub StartSize: f32,
    pub Type: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSun<'a> {
    pub HDRColorScale: f32,
    pub angle: f32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub material: &'a str,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub overlaycolor: Color,
    pub overlaymaterial: &'a str,
    pub overlaysize: i32,
    pub pitch: f32,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub size: i32,
    pub targetname: &'a str,
    pub use_angles: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvTexturetoggle {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvTonemapController {}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvTracer {
    pub delay: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvViewpunch {
    pub punchangle: Vector,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvWind<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub gustdirchange: i32,
    pub gustduration: f32,
    pub gustsound: &'a str,
    pub maxgust: i32,
    pub maxgustdelay: f32,
    pub maxs: Vector,
    pub maxwind: i32,
    pub mingust: i32,
    pub mingustdelay: f32,
    pub mins: Vector,
    pub minwind: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvZoom {
    pub FOV: i32,
    pub Rate: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EventQueueSaveloadProxy {}
#[derive(Debug, Clone, Deserialize)]
pub struct EyeballBoss {}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorClass<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    pub filterclass: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorMassGreater {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    pub filtermass: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorName<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorTeam {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    pub filterteam: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorTfteam<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    pub controlpoint: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterBase {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterCombineballType {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    pub balltype: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterDamageType {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    pub damagetype: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterEnemy<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    pub filter_max_per_enemy: i32,
    pub filter_outer_radius: f32,
    pub filter_radius: f32,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterMulti<'a> {
    pub Filter01: &'a str,
    pub Filter02: &'a str,
    pub Filter03: &'a str,
    pub Filter04: &'a str,
    pub Filter05: &'a str,
    pub FilterType: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterTfBotHasTag<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub require_all_tags: bool,
    pub tags: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterTfClass {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    pub tfclass: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterTfCondition {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    pub condition: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterTfDamagedByWeaponInSlot {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
    pub weaponSlot: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterTfPlayerCanCap {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Negated: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Firesmoke {}
#[derive(Debug, Clone, Deserialize)]
pub struct Fish<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FloorturretTipcontroller {}
#[derive(Debug, Clone, Deserialize)]
pub struct FunCBaseFlex {}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncAchievement<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
    pub zone_id: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncAreaportal {
    pub PortalVersion: i32,
    pub StartOpen: i32,
    pub portalnumber: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncAreaportalwindow<'a> {
    pub BackgroundBModel: &'a str,
    pub FadeDist: f32,
    pub FadeStartDist: f32,
    pub PortalVersion: i32,
    pub TranslucencyLimit: f32,
    pub portalnumber: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBreakable<'a> {
    pub PerformanceMode: i32,
    pub PressureDelay: f32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub explodemagnitude: i32,
    pub explosion: i32,
    pub gibdir: Vector,
    pub gibmodel: &'a str,
    pub material: i32,
    pub maxs: Vector,
    pub minhealthdmg: i32,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub propdata: i32,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub spawnobject: i32,
    pub targetname: &'a str,
    pub team_number: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBreakableSurf<'a> {
    pub PerformanceMode: i32,
    pub PressureDelay: f32,
    pub error: i32,
    pub explodemagnitude: i32,
    pub explosion: i32,
    pub fragility: i32,
    pub gibdir: Vector,
    pub gibmodel: &'a str,
    pub lowerleft: Vector,
    pub lowerright: Vector,
    pub material: i32,
    pub minhealthdmg: i32,
    pub propdata: i32,
    pub spawnobject: i32,
    pub surfacetype: i32,
    pub team_number: i32,
    pub upperleft: Vector,
    pub upperright: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBrush<'a> {
    pub Solidity: i32,
    pub StartDisabled: i32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub excludednpc: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub invert_exclusion: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub solidbsp: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBulletshield<'a> {
    pub Solidity: i32,
    pub StartDisabled: i32,
    pub excludednpc: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub invert_exclusion: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub solidbsp: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncButton<'a> {
    pub master: &'a str,
    pub movedir: Vector,
    pub sounds: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncCapturezone {
    pub CapturePoint: i32,
    pub capture_delay: f32,
    pub capture_delay_offset: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub shouldBlock: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncChangeclass<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncClipVphysics<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub filtername: &'a str,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncCombineBallSpawner<'a> {
    pub angles: Angles,
    pub ballcount: i32,
    pub ballradius: f32,
    pub ballrespawntime: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub maxspeed: f32,
    pub mins: Vector,
    pub minspeed: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncConveyor {
    pub movedir: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncCroc<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub croc_model: &'a str,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDoor<'a> {
    pub WaveHeight: f32,
    pub chainstodoor: &'a str,
    pub dmg: f32,
    pub filtername: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub forceclosed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoredebris: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub loopmovesound: bool,
    pub master: &'a str,
    pub movedir: Vector,
    pub spawnpos: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDoorRotating<'a> {
    pub WaveHeight: f32,
    pub chainstodoor: &'a str,
    pub dmg: f32,
    pub filtername: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub forceclosed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoredebris: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub loopmovesound: bool,
    pub movedir: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub solidbsp: bool,
    pub spawnpos: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDustcloud {
    pub Color: Color,
    pub DistMax: i32,
    pub FallSpeed: f32,
    pub LifetimeMax: i32,
    pub LifetimeMin: i32,
    pub SizeMax: f32,
    pub SizeMin: f32,
    pub SpawnRate: i32,
    pub SpeedMax: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDustmotes {
    pub Color: Color,
    pub DistMax: i32,
    pub FallSpeed: f32,
    pub LifetimeMax: i32,
    pub LifetimeMin: i32,
    pub SizeMax: f32,
    pub SizeMin: f32,
    pub SpawnRate: i32,
    pub SpeedMax: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncExtinguishercharger<'a> {
    pub master: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncFishPool<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub fish_count: i32,
    pub maxs: Vector,
    pub mins: Vector,
    pub model: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncFlagAlert<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub alert_delay: i32,
    pub filtername: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub playsound: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncFlagdetectionzone<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub alarm: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncForcefield<'a> {
    pub Solidity: i32,
    pub StartDisabled: i32,
    pub excludednpc: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub invert_exclusion: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub solidbsp: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncFriction<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncGuntarget<'a> {
    pub master: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncHealthcharger<'a> {
    pub dmdelay: i32,
    pub master: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncIllusionary<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncLadderendpoint<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncLod<'a> {
    pub Solid: i32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncLookdoor {
    pub BlockDamage: f32,
    pub FieldOfView: f32,
    pub MoveDistance: f32,
    pub ProximityDistance: f32,
    pub ProximityOffset: f32,
    pub StartPosition: f32,
    pub movedir: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncMonitor<'a> {
    pub Solidity: i32,
    pub StartDisabled: i32,
    pub excludednpc: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub invert_exclusion: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub solidbsp: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncMovelinear<'a> {
    pub BlockDamage: f32,
    pub MoveDistance: f32,
    pub StartPosition: f32,
    pub master: &'a str,
    pub movedir: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNavAvoid<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub start_disabled: bool,
    pub tags: &'a str,
    pub team: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNavAvoidanceObstacle<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNavBlocker<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
    pub teamToBlock: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNavPrefer<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub start_disabled: bool,
    pub tags: &'a str,
    pub team: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNavPrerequisite<'a> {
    pub Entity: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub Task: i32,
    pub Value: f32,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNobuild<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AllowDispenser: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub AllowSentry: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub AllowTeleporters: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DestroyBuildings: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncNogrenades<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncOccluder<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartActive: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub occludernumber: i32,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPasstimeGoal {
    pub points: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPasstimeGoalieZone<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPasstimeNoBallZone<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysbox<'a> {
    pub Damagetype: i32,
    pub PerformanceMode: i32,
    pub PressureDelay: f32,
    pub damagetoenablemotion: i32,
    pub explodemagnitude: i32,
    pub explosion: i32,
    pub forcetoenablemotion: f32,
    pub gibdir: Vector,
    pub gibmodel: &'a str,
    pub massScale: f32,
    pub material: i32,
    pub minhealthdmg: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub notsolid: bool,
    pub overridescript: &'a str,
    pub preferredcarryangles: Vector,
    pub propdata: i32,
    pub spawnobject: i32,
    pub team_number: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysboxMultiplayer<'a> {
    pub Damagetype: i32,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub massScale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub notsolid: bool,
    pub overridescript: &'a str,
    pub preferredcarryangles: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPlat {
    pub height: f32,
    pub lip: f32,
    pub movesnd: i32,
    pub stopsnd: i32,
    pub volume: f32,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPlatrot {}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPowerupvolume {}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPrecipitation<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub preciptype: i32,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncProprrespawnzone<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPushable<'a> {
    pub PerformanceMode: i32,
    pub PressureDelay: f32,
    pub explodemagnitude: i32,
    pub explosion: i32,
    pub gibdir: Vector,
    pub gibmodel: &'a str,
    pub material: i32,
    pub minhealthdmg: i32,
    pub propdata: i32,
    pub spawnobject: i32,
    pub team_number: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRecharge<'a> {
    pub dmdelay: i32,
    pub master: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncReflectiveGlass<'a> {
    pub Solidity: i32,
    pub StartDisabled: i32,
    pub excludednpc: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub invert_exclusion: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub solidbsp: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRegenerate<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub associatedmodel: &'a str,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRespawnflag<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRespawnroom {}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRespawnroomvisualizer<'a> {
    pub Solidity: i32,
    pub StartDisabled: i32,
    pub excludednpc: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub invert_exclusion: bool,
    pub respawnroomname: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub solid_to_enemies: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub solidbsp: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotButton {
    pub movedir: Vector,
    pub sounds: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotating<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub dmg: f32,
    pub maxs: Vector,
    pub maxspeed: f32,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub solidbsp: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncSmokevolume<'a> {
    pub Color1: Color,
    pub Color2: Color,
    pub Density: f32,
    pub DensityRampSpeed: f32,
    pub Material: &'a str,
    pub MovementSpeed: f32,
    pub ParticleDrawWidth: f32,
    pub ParticleSpacingDistance: f32,
    pub RotationSpeed: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncSuggestedBuild<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub face_entity: &'a str,
    pub face_entity_fov: f32,
    pub filtername: &'a str,
    pub object_type: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTank<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub LeadTarget: bool,
    pub ammo_count: i32,
    pub ammotype: &'a str,
    pub bullet: i32,
    pub bullet_damage: i32,
    pub bullet_damage_vs_player: i32,
    pub control_volume: &'a str,
    pub effecthandling: i32,
    pub firerate: f32,
    pub firespread: i32,
    pub gun_barrel_attach: &'a str,
    pub gun_base_attach: &'a str,
    pub gun_pitch_pose_center: f32,
    pub gun_pitch_pose_param: &'a str,
    pub gun_yaw_pose_center: f32,
    pub gun_yaw_pose_param: &'a str,
    pub ignoregraceupto: f32,
    pub master: &'a str,
    pub maxRange: f32,
    pub minRange: f32,
    pub npc_man_point: &'a str,
    pub persistence: f32,
    pub persistence2: f32,
    pub pitchrange: f32,
    pub pitchrate: f32,
    pub pitchtolerance: f32,
    pub playergraceperiod: f32,
    pub playerlocktimebeforefire: f32,
    pub spriteflash: &'a str,
    pub spritescale: f32,
    pub spritesmoke: &'a str,
    pub yawrange: f32,
    pub yawrate: f32,
    pub yawtolerance: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTankCombineCannon {}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTankairboatgun<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub LeadTarget: bool,
    pub airboat_gun_model: &'a str,
    pub ammo_count: i32,
    pub ammotype: &'a str,
    pub bullet: i32,
    pub bullet_damage: i32,
    pub bullet_damage_vs_player: i32,
    pub control_volume: &'a str,
    pub effecthandling: i32,
    pub firerate: f32,
    pub firespread: i32,
    pub gun_barrel_attach: &'a str,
    pub gun_base_attach: &'a str,
    pub gun_pitch_pose_center: f32,
    pub gun_pitch_pose_param: &'a str,
    pub gun_yaw_pose_center: f32,
    pub gun_yaw_pose_param: &'a str,
    pub ignoregraceupto: f32,
    pub master: &'a str,
    pub maxRange: f32,
    pub minRange: f32,
    pub npc_man_point: &'a str,
    pub persistence: f32,
    pub persistence2: f32,
    pub pitchrange: f32,
    pub pitchrate: f32,
    pub pitchtolerance: f32,
    pub playergraceperiod: f32,
    pub playerlocktimebeforefire: f32,
    pub spriteflash: &'a str,
    pub spritescale: f32,
    pub spritesmoke: &'a str,
    pub yawrange: f32,
    pub yawrate: f32,
    pub yawtolerance: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTankapcrocket<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub LeadTarget: bool,
    pub ammo_count: i32,
    pub ammotype: &'a str,
    pub bullet: i32,
    pub bullet_damage: i32,
    pub bullet_damage_vs_player: i32,
    pub burstcount: i32,
    pub control_volume: &'a str,
    pub effecthandling: i32,
    pub firerate: f32,
    pub firespread: i32,
    pub gun_barrel_attach: &'a str,
    pub gun_base_attach: &'a str,
    pub gun_pitch_pose_center: f32,
    pub gun_pitch_pose_param: &'a str,
    pub gun_yaw_pose_center: f32,
    pub gun_yaw_pose_param: &'a str,
    pub ignoregraceupto: f32,
    pub master: &'a str,
    pub maxRange: f32,
    pub minRange: f32,
    pub npc_man_point: &'a str,
    pub persistence: f32,
    pub persistence2: f32,
    pub pitchrange: f32,
    pub pitchrate: f32,
    pub pitchtolerance: f32,
    pub playergraceperiod: f32,
    pub playerlocktimebeforefire: f32,
    pub rocketspeed: f32,
    pub spriteflash: &'a str,
    pub spritescale: f32,
    pub spritesmoke: &'a str,
    pub yawrange: f32,
    pub yawrate: f32,
    pub yawtolerance: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTanklaser<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub LeadTarget: bool,
    pub ammo_count: i32,
    pub ammotype: &'a str,
    pub bullet: i32,
    pub bullet_damage: i32,
    pub bullet_damage_vs_player: i32,
    pub control_volume: &'a str,
    pub effecthandling: i32,
    pub firerate: f32,
    pub firespread: i32,
    pub gun_barrel_attach: &'a str,
    pub gun_base_attach: &'a str,
    pub gun_pitch_pose_center: f32,
    pub gun_pitch_pose_param: &'a str,
    pub gun_yaw_pose_center: f32,
    pub gun_yaw_pose_param: &'a str,
    pub ignoregraceupto: f32,
    pub laserentity: &'a str,
    pub master: &'a str,
    pub maxRange: f32,
    pub minRange: f32,
    pub npc_man_point: &'a str,
    pub persistence: f32,
    pub persistence2: f32,
    pub pitchrange: f32,
    pub pitchrate: f32,
    pub pitchtolerance: f32,
    pub playergraceperiod: f32,
    pub playerlocktimebeforefire: f32,
    pub spriteflash: &'a str,
    pub spritescale: f32,
    pub spritesmoke: &'a str,
    pub yawrange: f32,
    pub yawrate: f32,
    pub yawtolerance: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTankmortar<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub LeadTarget: bool,
    pub ammo_count: i32,
    pub ammotype: &'a str,
    pub bullet: i32,
    pub bullet_damage: i32,
    pub bullet_damage_vs_player: i32,
    pub control_volume: &'a str,
    pub effecthandling: i32,
    pub firedelay: f32,
    pub fireendsound: &'a str,
    pub firerate: f32,
    pub firespread: i32,
    pub firestartsound: &'a str,
    pub gun_barrel_attach: &'a str,
    pub gun_base_attach: &'a str,
    pub gun_pitch_pose_center: f32,
    pub gun_pitch_pose_param: &'a str,
    pub gun_yaw_pose_center: f32,
    pub gun_yaw_pose_param: &'a str,
    pub iMagnitude: i32,
    pub ignoregraceupto: f32,
    pub incomingsound: &'a str,
    pub master: &'a str,
    pub maxRange: f32,
    pub minRange: f32,
    pub npc_man_point: &'a str,
    pub persistence: f32,
    pub persistence2: f32,
    pub pitchrange: f32,
    pub pitchrate: f32,
    pub pitchtolerance: f32,
    pub playergraceperiod: f32,
    pub playerlocktimebeforefire: f32,
    pub spriteflash: &'a str,
    pub spritescale: f32,
    pub spritesmoke: &'a str,
    pub yawrange: f32,
    pub yawrate: f32,
    pub yawtolerance: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTankphyscannister<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub LeadTarget: bool,
    pub ammo_count: i32,
    pub ammotype: &'a str,
    pub barrel_volume: &'a str,
    pub bullet: i32,
    pub bullet_damage: i32,
    pub bullet_damage_vs_player: i32,
    pub control_volume: &'a str,
    pub effecthandling: i32,
    pub firerate: f32,
    pub firespread: i32,
    pub gun_barrel_attach: &'a str,
    pub gun_base_attach: &'a str,
    pub gun_pitch_pose_center: f32,
    pub gun_pitch_pose_param: &'a str,
    pub gun_yaw_pose_center: f32,
    pub gun_yaw_pose_param: &'a str,
    pub ignoregraceupto: f32,
    pub master: &'a str,
    pub maxRange: f32,
    pub minRange: f32,
    pub npc_man_point: &'a str,
    pub persistence: f32,
    pub persistence2: f32,
    pub pitchrange: f32,
    pub pitchrate: f32,
    pub pitchtolerance: f32,
    pub playergraceperiod: f32,
    pub playerlocktimebeforefire: f32,
    pub spriteflash: &'a str,
    pub spritescale: f32,
    pub spritesmoke: &'a str,
    pub yawrange: f32,
    pub yawrate: f32,
    pub yawtolerance: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTankpulselaser {
    pub PulseColor: Color,
    pub PulseLag: f32,
    pub PulseLife: f32,
    pub PulseSpeed: f32,
    pub PulseWidth: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTankrocket<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub LeadTarget: bool,
    pub ammo_count: i32,
    pub ammotype: &'a str,
    pub bullet: i32,
    pub bullet_damage: i32,
    pub bullet_damage_vs_player: i32,
    pub control_volume: &'a str,
    pub effecthandling: i32,
    pub firerate: f32,
    pub firespread: i32,
    pub gun_barrel_attach: &'a str,
    pub gun_base_attach: &'a str,
    pub gun_pitch_pose_center: f32,
    pub gun_pitch_pose_param: &'a str,
    pub gun_yaw_pose_center: f32,
    pub gun_yaw_pose_param: &'a str,
    pub ignoregraceupto: f32,
    pub master: &'a str,
    pub maxRange: f32,
    pub minRange: f32,
    pub npc_man_point: &'a str,
    pub persistence: f32,
    pub persistence2: f32,
    pub pitchrange: f32,
    pub pitchrate: f32,
    pub pitchtolerance: f32,
    pub playergraceperiod: f32,
    pub playerlocktimebeforefire: f32,
    pub rocketspeed: f32,
    pub spriteflash: &'a str,
    pub spritescale: f32,
    pub spritesmoke: &'a str,
    pub yawrange: f32,
    pub yawrate: f32,
    pub yawtolerance: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTanktrain {
    pub ManualAccelSpeed: f32,
    pub ManualDecelSpeed: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ManualSpeedChanges: bool,
    pub MoveSoundMaxPitch: i32,
    pub MoveSoundMaxTime: f32,
    pub MoveSoundMinPitch: i32,
    pub MoveSoundMinTime: f32,
    pub bank: f32,
    pub dmg: f32,
    pub height: f32,
    pub orientationtype: i32,
    pub startspeed: f32,
    pub velocitytype: i32,
    pub volume: i32,
    pub wheels: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTfbotHint<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub hint: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
    pub team: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTrackautochange {}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTrackchange {}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTracktrain<'a> {
    pub ManualAccelSpeed: f32,
    pub ManualDecelSpeed: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ManualSpeedChanges: bool,
    pub MoveSoundMaxPitch: i32,
    pub MoveSoundMaxTime: f32,
    pub MoveSoundMinPitch: i32,
    pub MoveSoundMinTime: f32,
    pub angles: Angles,
    pub bank: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub dmg: f32,
    pub height: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub orientationtype: i32,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub startspeed: f32,
    pub targetname: &'a str,
    pub velocitytype: i32,
    pub volume: i32,
    pub wheels: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTrain {
    pub dmg: f32,
    pub height: f32,
    pub lip: f32,
    pub movesnd: i32,
    pub stopsnd: i32,
    pub volume: f32,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTraincontrols<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncUpgradestation<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
    pub start_disabled: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncUseableladder<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub ladderSurfaceProperties: &'a str,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub point0: Vector,
    pub point1: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncVehicleclip<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWall<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWallToggle {}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWater<'a> {
    pub WaveHeight: f32,
    pub chainstodoor: &'a str,
    pub dmg: f32,
    pub filtername: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub forceclosed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoredebris: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub loopmovesound: bool,
    pub master: &'a str,
    pub movedir: Vector,
    pub spawnpos: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWaterAnalog<'a> {
    pub BlockDamage: f32,
    pub MoveDistance: f32,
    pub StartPosition: f32,
    pub master: &'a str,
    pub movedir: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWeightButton<'a> {
    pub WeightToActivate: f32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameEnd {}
#[derive(Debug, Clone, Deserialize)]
pub struct GameForcerespawn {}
#[derive(Debug, Clone, Deserialize)]
pub struct GameGibManager<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub allownewgibs: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxpieces: i32,
    pub maxpiecesdx8: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameIntroViewpoint<'a> {
    pub event_data_int: i32,
    pub event_delay: f32,
    pub event_to_fire: &'a str,
    pub fov: f32,
    pub hint_message: &'a str,
    pub step_number: i32,
    pub time_delay: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GamePlayerEquip {}
#[derive(Debug, Clone, Deserialize)]
pub struct GamePlayerHurt {
    pub dmg: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GamePlayerTeam {}
#[derive(Debug, Clone, Deserialize)]
pub struct GameRagdollManager<'a> {
    pub MaxRagdollCount: i32,
    pub MaxRagdollCountDX8: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub SaveImportant: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameRoundWin {
    #[serde(deserialize_with = "deserialize_bool")]
    pub force_map_reset: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub switch_teams: bool,
    pub win_reason: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameScore {
    pub points: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameTeamSet {}
#[derive(Debug, Clone, Deserialize)]
pub struct GameText<'a> {
    pub channel: i32,
    pub effect: i32,
    pub fadein: f32,
    pub fadeout: f32,
    pub fxtime: f32,
    pub holdtime: f32,
    pub message: &'a str,
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameTextTf<'a> {
    pub background: i32,
    pub display_to_team: i32,
    pub icon: &'a str,
    pub message: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameUi<'a> {
    pub FieldOfView: f32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameWeaponManager<'a> {
    pub ammomod: f32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxpieces: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
    pub weaponname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameZonePlayer {}
#[derive(Debug, Clone, Deserialize)]
pub struct GenericActor<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontUseSpeechSemaphore: bool,
    pub ExpressionOverride: &'a str,
    pub hull_name: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Ghost {}
#[derive(Debug, Clone, Deserialize)]
pub struct Gib<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Gibshooter<'a> {
    pub LightingOrigin: &'a str,
    pub Simulation: i32,
    pub angles: Angles,
    pub delay: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub gibangles: Vector,
    pub gibanglevelocity: f32,
    pub m_flGibLife: f32,
    pub m_flVariance: f32,
    pub m_flVelocity: f32,
    pub m_iGibs: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nogibshadows: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GravityPellet<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Grenade {}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadeAr2 {}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadeBeam {}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadeBeamChaser<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadeBeerbottle {}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadeEnergy {}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadeHelicopter {}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadeHomer {}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadeMolotov {}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadePathfollower {}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadeProjectile {}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadeRockbb {}
#[derive(Debug, Clone, Deserialize)]
pub struct GrenadeSpit {}
#[derive(Debug, Clone, Deserialize)]
pub struct HalloweenFortuneTeller<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub blue_teleport: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub red_teleport: &'a str,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HalloweenSoulsPack<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HalloweenZapper<'a> {
    pub ParticleEffect: &'a str,
    pub ZapperType: i32,
    pub touch_trigger: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HammerUpdateignorelist<'a> {
    pub IgnoredName01: &'a str,
    pub IgnoredName02: &'a str,
    pub IgnoredName03: &'a str,
    pub IgnoredName04: &'a str,
    pub IgnoredName05: &'a str,
    pub IgnoredName06: &'a str,
    pub IgnoredName07: &'a str,
    pub IgnoredName08: &'a str,
    pub IgnoredName09: &'a str,
    pub IgnoredName10: &'a str,
    pub IgnoredName11: &'a str,
    pub IgnoredName12: &'a str,
    pub IgnoredName13: &'a str,
    pub IgnoredName14: &'a str,
    pub IgnoredName15: &'a str,
    pub IgnoredName16: &'a str,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HandleDummy<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HandleTest<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HeadlessHatman {}
#[derive(Debug, Clone, Deserialize)]
pub struct HelicopterChunk<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HightowerTeleportVortex<'a> {
    pub lifetime: f32,
    pub target_base_name: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Hl2Gamerules {}
#[derive(Debug, Clone, Deserialize)]
pub struct Hl2SurvivalGamerules {}
#[derive(Debug, Clone, Deserialize)]
pub struct Hl2mpBot {}
#[derive(Debug, Clone, Deserialize)]
pub struct Hl2mpGamerules {}
#[derive(Debug, Clone, Deserialize)]
pub struct Hl2mpRagdoll {}
#[derive(Debug, Clone, Deserialize)]
pub struct HunterFlechette<'a> {
    pub Damagetype: i32,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub overridescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HydraImpale<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoApcMissileHint<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoApcMissileHint<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoCameraLink<'a> {
    pub PointCamera: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoConstraintAnchor {
    pub massScale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoDarknessmodeLightsource<'a> {
    pub LightRadius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoHint {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoIntermission {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoLadderDismount<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoLandmark<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoLightingRelative<'a> {
    pub LightingLandmark: &'a str,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoMassCenter {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNode {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNodeAir {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNodeAirHint {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNodeClimb {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNodeHint {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNodeLink<'a> {
    pub AllowUse: &'a str,
    pub endnode: i32,
    pub initialstate: i32,
    pub linktype: i32,
    pub startnode: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNodeLinkController<'a> {
    pub AllowUse: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub InvertAllow: bool,
    pub initialstate: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub useairlinkradius: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNpcSpawnDestination<'a> {
    pub RenameNPC: &'a str,
    pub ReuseDelay: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNull<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoObserverPoint<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub associated_team_entity: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub defaultwelcome: bool,
    pub fov: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub match_summary: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoOverlayAccessor {
    pub OverlayID: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoParticleSystem<'a> {
    pub angles: Angles,
    pub cpoint1: &'a str,
    pub cpoint10: &'a str,
    pub cpoint11: &'a str,
    pub cpoint12: &'a str,
    pub cpoint13: &'a str,
    pub cpoint14: &'a str,
    pub cpoint15: &'a str,
    pub cpoint16: &'a str,
    pub cpoint17: &'a str,
    pub cpoint18: &'a str,
    pub cpoint19: &'a str,
    pub cpoint2: &'a str,
    pub cpoint20: &'a str,
    pub cpoint21: &'a str,
    pub cpoint22: &'a str,
    pub cpoint23: &'a str,
    pub cpoint24: &'a str,
    pub cpoint25: &'a str,
    pub cpoint26: &'a str,
    pub cpoint27: &'a str,
    pub cpoint28: &'a str,
    pub cpoint29: &'a str,
    pub cpoint3: &'a str,
    pub cpoint30: &'a str,
    pub cpoint31: &'a str,
    pub cpoint32: &'a str,
    pub cpoint33: &'a str,
    pub cpoint34: &'a str,
    pub cpoint35: &'a str,
    pub cpoint36: &'a str,
    pub cpoint37: &'a str,
    pub cpoint38: &'a str,
    pub cpoint39: &'a str,
    pub cpoint4: &'a str,
    pub cpoint40: &'a str,
    pub cpoint41: &'a str,
    pub cpoint42: &'a str,
    pub cpoint43: &'a str,
    pub cpoint44: &'a str,
    pub cpoint45: &'a str,
    pub cpoint46: &'a str,
    pub cpoint47: &'a str,
    pub cpoint48: &'a str,
    pub cpoint49: &'a str,
    pub cpoint5: &'a str,
    pub cpoint50: &'a str,
    pub cpoint51: &'a str,
    pub cpoint52: &'a str,
    pub cpoint53: &'a str,
    pub cpoint54: &'a str,
    pub cpoint55: &'a str,
    pub cpoint56: &'a str,
    pub cpoint57: &'a str,
    pub cpoint58: &'a str,
    pub cpoint59: &'a str,
    pub cpoint6: &'a str,
    pub cpoint60: &'a str,
    pub cpoint61: &'a str,
    pub cpoint62: &'a str,
    pub cpoint63: &'a str,
    pub cpoint7: &'a str,
    pub cpoint8: &'a str,
    pub cpoint9: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub effect_name: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub flag_as_weather: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub start_active: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPasstimeBallSpawn {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerCombine<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerCounterterrorist<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerDeathmatch<'a> {
    pub master: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerRebel<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerStart<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerTeamspawn {
    pub StartDisabled: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerTeamspawn<'a> {
    pub MatchSummary: i32,
    pub SpawnMode: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub controlpoint: &'a str,
    pub round_bluespawn: &'a str,
    pub round_redspawn: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerTeamspawnOld {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerTerrorist<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPopulator {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPowerupSpawn {
    #[serde(deserialize_with = "deserialize_bool")]
    pub disabled: bool,
    pub team: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoProjecteddecal {
    pub Distance: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoRadarTarget {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub mode: i32,
    pub r#type: i32,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoRadialLinkController<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub radius: f32,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoSnipertarget<'a> {
    pub groupname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTarget {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTargetCommandPoint {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTargetGunshipcrash {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTargetHelicopterCrash<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTargetImmolator<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTargetVehicleTransition {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTeleportDestination<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTeleporterCountdown {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTfTeamcheck {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTfgoal {}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoVehicleGroundspawn {
    pub StartDisabled: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Infodecal {
    #[serde(deserialize_with = "deserialize_bool")]
    pub LowPriority: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InstancedScriptedScene<'a> {
    pub ResumeSceneFile: &'a str,
    pub SceneFile: &'a str,
    pub busyactor: i32,
    pub onplayerdeath: i32,
    pub target1: &'a str,
    pub target2: &'a str,
    pub target3: &'a str,
    pub target4: &'a str,
    pub target5: &'a str,
    pub target6: &'a str,
    pub target7: &'a str,
    pub target8: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmo357 {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmo357Large {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmoAr2 {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmoAr2Altfire {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmoAr2Large {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmoCrate<'a> {
    pub AmmoType: i32,
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmoCrossbow {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmoPistol {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmoPistolLarge {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmoSmg1 {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmoSmg1Grenade {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmoSmg1Large {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmopackFull<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AutoMaterialize: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub powerup_model: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmopackMedium {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAmmopackSmall {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAntidote {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemAr2Grenade {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemArmor<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AutoMaterialize: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub powerup_model: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemBattery {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemBonuspack {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemBoxBuckshot {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemBoxFlareRounds {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemBoxLrounds {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemBoxMrounds {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemBoxSniperRounds {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemBoxSrounds {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemCurrencypackCustom {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemCurrencypackLarge<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AutoMaterialize: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub powerup_model: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemCurrencypackMedium {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemCurrencypackSmall {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemDynamicResupply {
    pub DesiredAmmo357: f32,
    pub DesiredAmmoAR2: f32,
    pub DesiredAmmoAR2_AltFire: f32,
    pub DesiredAmmoBuckshot: f32,
    pub DesiredAmmoCrossbow: f32,
    pub DesiredAmmoGrenade: f32,
    pub DesiredAmmoPistol: f32,
    pub DesiredAmmoRPG_Round: f32,
    pub DesiredAmmoSMG1: f32,
    pub DesiredAmmoSMG1_Grenade: f32,
    pub DesiredArmor: f32,
    pub DesiredHealth: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemFlareRound {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemGrenadepack<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AutoMaterialize: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub powerup_model: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemGrubnugget {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemHealthammokit {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemHealthcharger<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub dmdelay: i32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemHealthkit {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemHealthkitFull {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemHealthkitMedium {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemHealthkitSmall {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemHealthvial {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemItemCrate<'a> {
    pub CrateAppearance: i32,
    pub CrateType: i32,
    pub Damagetype: i32,
    pub ItemClass: &'a str,
    pub ItemCount: i32,
    pub SpecificResupply: &'a str,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub overridescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemLargeBoxLrounds {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemLargeBoxMrounds {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemLargeBoxSrounds {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemLongjump {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemMlGrenade {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemPowerupCrit {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemPowerupRune<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AutoMaterialize: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub powerup_model: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemPowerupRuneTemp<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AutoMaterialize: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub powerup_model: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemPowerupUber {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemRpgRound {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemSecurity {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemSodacan<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemSuit {}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemSuitcharger<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub dmdelay: i32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemTeamflag<'a> {
    pub GameType: i32,
    pub NeutralType: i32,
    pub PointValue: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ReturnBetweenWaves: bool,
    pub ReturnTime: i32,
    pub ScoringType: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ShotClockMode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub VisibleWhenDisabled: bool,
    pub flag_icon: &'a str,
    pub flag_model: &'a str,
    pub flag_paper: &'a str,
    pub flag_trail: &'a str,
    pub tags: &'a str,
    pub trail_effect: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemTeamflagReturnIcon<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ItemTfgoal {}
#[derive(Debug, Clone, Deserialize)]
pub struct KeyframeRope<'a> {
    pub NextKey: &'a str,
    pub ScrollSpeed: f32,
    pub Slack: i32,
    pub Subdiv: i32,
    pub TextureScale: f32,
    pub Width: f32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct KeyframeTrack<'a> {
    pub MoveSpeed: f32,
    pub NextKey: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Light<'a> {
    pub defaultstyle: i32,
    pub pattern: &'a str,
    pub pitch: Angles,
    pub style: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LightDynamic<'a> {
    pub _light: Color,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub pitch: Angles,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub spawnflags: i32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LightEnvironment<'a> {
    pub defaultstyle: i32,
    pub pattern: &'a str,
    pub pitch: Angles,
    pub style: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LightGlspot<'a> {
    pub defaultstyle: i32,
    pub pattern: &'a str,
    pub pitch: Angles,
    pub style: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LightSpot<'a> {
    pub defaultstyle: i32,
    pub pattern: &'a str,
    pub pitch: Angles,
    pub style: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LocalName {}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicAchievement<'a> {
    pub AchievementEvent: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicActiveAutosave {
    pub DangerousTime: f32,
    pub MinHitPointsToCommit: i32,
    pub MinimumHitPoints: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub NewLevelUnit: bool,
    pub TimeToTrigger: f32,
    pub TriggerHitPoints: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicAuto<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub globalstate: &'a str,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicAutosave {
    pub MinHitPointsToCommit: i32,
    pub MinimumHitPoints: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub NewLevelUnit: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicBranch {
    #[serde(deserialize_with = "deserialize_bool")]
    pub InitialValue: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicBranchListener<'a> {
    pub Branch01: &'a str,
    pub Branch02: &'a str,
    pub Branch03: &'a str,
    pub Branch04: &'a str,
    pub Branch05: &'a str,
    pub Branch06: &'a str,
    pub Branch07: &'a str,
    pub Branch08: &'a str,
    pub Branch09: &'a str,
    pub Branch10: &'a str,
    pub Branch11: &'a str,
    pub Branch12: &'a str,
    pub Branch13: &'a str,
    pub Branch14: &'a str,
    pub Branch15: &'a str,
    pub Branch16: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicCase<'a> {
    pub Case01: &'a str,
    pub Case02: &'a str,
    pub Case03: &'a str,
    pub Case04: &'a str,
    pub Case05: &'a str,
    pub Case06: &'a str,
    pub Case07: &'a str,
    pub Case08: &'a str,
    pub Case09: &'a str,
    pub Case10: &'a str,
    pub Case11: &'a str,
    pub Case12: &'a str,
    pub Case13: &'a str,
    pub Case14: &'a str,
    pub Case15: &'a str,
    pub Case16: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicChoreographedScene<'a> {
    pub ResumeSceneFile: &'a str,
    pub SceneFile: &'a str,
    pub busyactor: i32,
    pub onplayerdeath: i32,
    pub target1: &'a str,
    pub target2: &'a str,
    pub target3: &'a str,
    pub target4: &'a str,
    pub target5: &'a str,
    pub target6: &'a str,
    pub target7: &'a str,
    pub target8: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicCollisionPair<'a> {
    pub attach1: &'a str,
    pub attach2: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicCompare {
    pub CompareValue: f32,
    pub InitialValue: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicGameMessage<'a> {
    pub text: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicLineto<'a> {
    pub source: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicMeasureMovement<'a> {
    pub MeasureReference: &'a str,
    pub MeasureTarget: &'a str,
    pub MeasureType: i32,
    pub TargetReference: &'a str,
    pub TargetScale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicMirrorMovement<'a> {
    pub MirrorRelative: &'a str,
    pub MirrorTarget: &'a str,
    pub RemoteTarget: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicMulticompare {
    pub IntegerValue: i32,
    pub ShouldComparetoValue: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicNavigation {}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicPlayerproxy {}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicProximity<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicRelay {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicSceneListManager<'a> {
    pub scene0: &'a str,
    pub scene1: &'a str,
    pub scene10: &'a str,
    pub scene11: &'a str,
    pub scene12: &'a str,
    pub scene13: &'a str,
    pub scene14: &'a str,
    pub scene15: &'a str,
    pub scene2: &'a str,
    pub scene3: &'a str,
    pub scene4: &'a str,
    pub scene5: &'a str,
    pub scene6: &'a str,
    pub scene7: &'a str,
    pub scene8: &'a str,
    pub scene9: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicScript<'a> {
    pub Group00: &'a str,
    pub Group01: &'a str,
    pub Group02: &'a str,
    pub Group03: &'a str,
    pub Group04: &'a str,
    pub Group05: &'a str,
    pub Group06: &'a str,
    pub Group07: &'a str,
    pub Group08: &'a str,
    pub Group09: &'a str,
    pub Group10: &'a str,
    pub Group11: &'a str,
    pub Group12: &'a str,
    pub Group13: &'a str,
    pub Group14: &'a str,
    pub Group16: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicTimer {
    pub RefireTime: f32,
    pub StartDisabled: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Lookdoorthinker {}
#[derive(Debug, Clone, Deserialize)]
pub struct MapClassName {}
#[derive(Debug, Clone, Deserialize)]
pub struct MapobjCartDispenser<'a> {
    pub touch_trigger: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MaterialModifyControl<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MathColorblend {
    pub colormax: Color,
    pub colormin: Color,
    pub inmax: f32,
    pub inmin: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MathCounter {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub max: f32,
    pub min: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MathRemap {
    pub in1: f32,
    pub in2: f32,
    pub out1: f32,
    pub out2: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Merasmus {}
#[derive(Debug, Clone, Deserialize)]
pub struct MerasmusDancer<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ModelStudio {}
#[derive(Debug, Clone, Deserialize)]
pub struct MomentaryDoor<'a> {
    pub BlockDamage: f32,
    pub MoveDistance: f32,
    pub StartPosition: f32,
    pub master: &'a str,
    pub movedir: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MomentaryRotButton {
    pub StartDirection: i32,
    pub StartPosition: f32,
    pub returnspeed: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub solidbsp: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MonsterFurniture<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontUseSpeechSemaphore: bool,
    pub ExpressionOverride: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MonsterGeneric<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MonsterMiniturret {
    pub turnrate: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MonsterMiniturret {
    pub turnrate: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MonsterResource<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MonsterSentry {
    pub turnrate: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MonsterSentry {
    pub turnrate: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MonsterTurret {
    pub turnrate: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MonsterTurret {
    pub turnrate: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Mortarshell<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MoveKeyframed<'a> {
    pub MoveSpeed: f32,
    pub NextKey: &'a str,
    pub PositionInterpolator: i32,
    pub RotationInterpolator: i32,
    pub TimeModifier: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MoveRope<'a> {
    pub NextKey: &'a str,
    pub ScrollSpeed: f32,
    pub Slack: i32,
    pub Subdiv: i32,
    pub TextureScale: f32,
    pub Width: f32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Multisource<'a> {
    pub globalstate: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MyBrushEntity<'a> {
    pub master: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MyLogicalEntity {
    pub threshold: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MyModelEntity<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MyNpc<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcAdvisor<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub levitategoal_bottom: &'a str,
    pub levitategoal_top: &'a str,
    pub levitationarea: &'a str,
    pub priority_grab_name: &'a str,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub staging_ent_names: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcAlyx {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AlwaysTransition: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontPickupWeapons: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ShouldHaveEMP: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcAlyx {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AlwaysTransition: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontPickupWeapons: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ShouldHaveEMP: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcAntlion {
    pub eludedist: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignorebugbait: bool,
    pub radius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startburrowed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub unburroweffects: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcAntlionGrub<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcAntlionTemplateMaker<'a> {
    pub CriterionDistance: i32,
    pub CriterionVisibility: i32,
    pub DestinationGroup: &'a str,
    pub MinSpawnDistance: i32,
    pub TemplateName: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub createspores: bool,
    pub fighttarget: &'a str,
    pub followtarget: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignorebugbait: bool,
    pub pool_max: i32,
    pub pool_regen_amount: i32,
    pub pool_regen_time: f32,
    pub pool_start: i32,
    pub radius: f32,
    pub spawngroup: &'a str,
    pub spawnradius: f32,
    pub spawntarget: &'a str,
    pub vehicledistance: f32,
    pub workerspawnrate: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcAntlionguard<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub allowbark: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub cavernbreed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub incavern: bool,
    pub shovetargets: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startburrowed: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcApcdriver<'a> {
    pub drivermaxspeed: f32,
    pub driverminspeed: f32,
    pub vehicle: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcAssassin<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcBarnacle<'a> {
    pub RestDist: f32,
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcBarnacleTongueTip<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcBarney {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AlwaysTransition: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontPickupWeapons: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcBlob<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcBreen<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontUseSpeechSemaphore: bool,
    pub ExpressionOverride: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcBullseye<'a> {
    pub additionalequipment: &'a str,
    pub autoaimradius: f32,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub minangle: f32,
    pub mindist: f32,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcBullsquid<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcCitizen<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AlwaysTransition: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontPickupWeapons: bool,
    pub ammoamount: i32,
    pub ammosupply: &'a str,
    pub citizentype: i32,
    pub denycommandconcept: &'a str,
    pub expressiontype: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub neverleaveplayersquad: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub notifynavfailblocked: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcClawscanner {
    #[serde(deserialize_with = "deserialize_bool")]
    pub NeverInspectPlayers: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub OnlyInspectPlayers: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ShouldInspect: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub SpotlightDisabled: bool,
    pub SpotlightLength: f32,
    pub SpotlightWidth: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcCombine<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontUseSpeechSemaphore: bool,
    pub ExpressionOverride: &'a str,
    pub NumGrenades: i32,
    pub pathfindingvariant: i32,
    pub tacticalvariant: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcCombineArmored {
    pub usemarch: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcCombineCamera<'a> {
    pub additionalequipment: &'a str,
    pub defaulttarget: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub innerradius: i32,
    pub minhealthdmg: i32,
    pub outerradius: i32,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcCombineCannon<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sightdist: f32,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcCombineS {
    pub NumGrenades: i32,
    pub pathfindingvariant: i32,
    pub tacticalvariant: i32,
    pub usemarch: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcCombinedropship<'a> {
    pub APCVehicleName: &'a str,
    pub CrateType: i32,
    pub Dustoff1: &'a str,
    pub Dustoff2: &'a str,
    pub Dustoff3: &'a str,
    pub Dustoff4: &'a str,
    pub Dustoff5: &'a str,
    pub Dustoff6: &'a str,
    pub GunRange: f32,
    pub InitialSpeed: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub Invulnerable: bool,
    pub LandTarget: &'a str,
    pub NPCTemplate: &'a str,
    pub NPCTemplate2: &'a str,
    pub NPCTemplate3: &'a str,
    pub NPCTemplate4: &'a str,
    pub NPCTemplate5: &'a str,
    pub NPCTemplate6: &'a str,
    pub RollermineTemplate: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcCombinegunship {
    pub InitialSpeed: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcConcussiongrenade {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcContactgrenade {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcCranedriver<'a> {
    pub drivermaxspeed: f32,
    pub driverminspeed: f32,
    pub releasepause: f32,
    pub vehicle: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcCrow<'a> {
    pub additionalequipment: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub deaf: bool,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcCscanner {
    #[serde(deserialize_with = "deserialize_bool")]
    pub NeverInspectPlayers: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub OnlyInspectPlayers: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ShouldInspect: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub SpotlightDisabled: bool,
    pub SpotlightLength: f32,
    pub SpotlightWidth: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcDog<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontUseSpeechSemaphore: bool,
    pub ExpressionOverride: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcEli<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontUseSpeechSemaphore: bool,
    pub ExpressionOverride: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcEnemyfinder<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcEnemyfinderCombinecannon<'a> {
    pub snaptoent: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcFastzombie {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcFastzombieTorso {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcFisherman {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AlwaysTransition: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontPickupWeapons: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcFurniture<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontUseSpeechSemaphore: bool,
    pub ExpressionOverride: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcGman {
    #[serde(deserialize_with = "deserialize_bool")]
    pub GameEndAlly: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcGrenadeBugbait {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcGrenadeFrag {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcGrenadeHopwire {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHandgrenade {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHeadcrab {
    #[serde(deserialize_with = "deserialize_bool")]
    pub startburrowed: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHeadcrabBlack {
    #[serde(deserialize_with = "deserialize_bool")]
    pub startburrowed: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHeadcrabFast {
    #[serde(deserialize_with = "deserialize_bool")]
    pub startburrowed: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHeadcrabPoison {
    #[serde(deserialize_with = "deserialize_bool")]
    pub startburrowed: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHeliAvoidbox<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHeliAvoidsphere<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub radius: f32,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHeliNobomb<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHelicopter<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AlwaysTransition: bool,
    pub GracePeriod: f32,
    pub InitialSpeed: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub NonCombat: bool,
    pub PatrolSpeed: f32,
    pub TransitionTarget: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHelicoptersensor<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHoundeye<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHunter<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontUseSpeechSemaphore: bool,
    pub ExpressionOverride: &'a str,
    pub FollowTarget: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHunterMaker<'a> {
    pub CriterionDistance: i32,
    pub CriterionVisibility: i32,
    pub DestinationGroup: &'a str,
    pub MinSpawnDistance: i32,
    pub TemplateName: &'a str,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcHydra<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcIchthyosaur<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcKleiner<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontUseSpeechSemaphore: bool,
    pub ExpressionOverride: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcLauncher<'a> {
    pub Damage: f32,
    pub DamageRadius: f32,
    pub FlySound: &'a str,
    pub Gravity: f32,
    pub HomingDelay: f32,
    pub HomingDuration: f32,
    pub HomingRampDown: f32,
    pub HomingRampUp: f32,
    pub HomingSpeed: f32,
    pub HomingStrength: i32,
    pub LaunchDelay: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub LaunchSmoke: bool,
    pub LaunchSound: &'a str,
    pub LaunchSpeed: f32,
    pub MaxRange: f32,
    pub MinRange: f32,
    pub MissileModel: &'a str,
    pub PathCornerName: &'a str,
    pub SmokeTrail: i32,
    pub SpinMagnitude: f32,
    pub SpinSpeed: f32,
    pub StartOn: i32,
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcMagnusson<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontUseSpeechSemaphore: bool,
    pub ExpressionOverride: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcMaker<'a> {
    pub IgnoreEntity: &'a str,
    pub MaxLiveChildren: i32,
    pub MaxNPCCount: i32,
    pub NPCHintGroup: &'a str,
    pub NPCSquadName: &'a str,
    pub NPCTargetname: &'a str,
    pub NPCType: &'a str,
    pub Relationship: &'a str,
    pub SpawnFrequency: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub additionalequipment: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcManhack {
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreclipbrushes: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcMetropolice<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontUseSpeechSemaphore: bool,
    pub ExpressionOverride: &'a str,
    pub manhacks: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub weapondrawn: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcMiniturret {
    pub turnrate: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcMissiledefense<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcMonk {
    #[serde(deserialize_with = "deserialize_bool")]
    pub GameEndAlly: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcMossman {
    #[serde(deserialize_with = "deserialize_bool")]
    pub GameEndAlly: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcNewnpc<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcPigeon {
    #[serde(deserialize_with = "deserialize_bool")]
    pub deaf: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcPoisonzombie {
    pub crabcount: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcPuppet<'a> {
    pub additionalequipment: &'a str,
    pub animationtarget: &'a str,
    pub attachmentname: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcRollermine {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartBuried: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub uniformsightdist: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcSatchel {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcSatchel {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcSeagull {
    #[serde(deserialize_with = "deserialize_bool")]
    pub deaf: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcSentry {
    pub turnrate: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcSniper<'a> {
    pub PaintInterval: f32,
    pub PaintIntervalVariance: f32,
    pub additionalequipment: &'a str,
    pub beambrightness: i32,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub misses: i32,
    pub shielddistance: f32,
    pub shieldradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub shootZombiesInChest: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcSpotlight<'a> {
    pub AlertSpeed: f32,
    pub IdleSpeed: f32,
    pub PitchMax: f32,
    pub PitchMin: f32,
    pub SpotlightLength: f32,
    pub SpotlightWidth: f32,
    pub YawRange: f32,
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcStalker {
    pub BeamPower: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcStrider {
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablephysics: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcTemplateMaker<'a> {
    pub CriterionDistance: i32,
    pub CriterionVisibility: i32,
    pub DestinationGroup: &'a str,
    pub IgnoreEntity: &'a str,
    pub MaxLiveChildren: i32,
    pub MaxNPCCount: i32,
    pub MinSpawnDistance: i32,
    pub SpawnFrequency: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub TemplateName: &'a str,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcTripmine {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcTripmine {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcTripwire {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcTurretCeiling {
    pub turnrate: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcTurretCeiling<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub minhealthdmg: i32,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcTurretFloor {
    pub SkinNumber: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcTurretGround<'a> {
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcVehicledriver<'a> {
    pub additionalequipment: &'a str,
    pub drivermaxspeed: f32,
    pub driverminspeed: f32,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub vehicle: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcVortigaunt {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AlwaysTransition: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ArmorRechargeEnabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DontPickupWeapons: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub HealthRegenerateEnabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcWpnscanner {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcZombie {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcZombieTorso {}
#[derive(Debug, Clone, Deserialize)]
pub struct NpcZombine {}
#[derive(Debug, Clone, Deserialize)]
pub struct ObjAttachmentSapper {}
#[derive(Debug, Clone, Deserialize)]
pub struct ObjDispenser<'a> {
    pub defaultupgrade: i32,
    pub touch_trigger: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ObjSentrygun {
    pub defaultupgrade: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ObjTeleporter<'a> {
    pub defaultupgrade: i32,
    pub matchingTeleporter: &'a str,
    pub teleporterType: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PasstimeBall<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PasstimeBall {}
#[derive(Debug, Clone, Deserialize)]
pub struct PasstimeLogic {
    pub ball_spawn_countdown: i32,
    pub max_pass_range: f32,
    pub num_sections: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PasstimeLogic {}
#[derive(Debug, Clone, Deserialize)]
pub struct PathCorner {
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PathCornerCrash {
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PathTrack<'a> {
    pub altpath: &'a str,
    pub orientationtype: i32,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PdDispenser {}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysBallsocket<'a> {
    pub attach1: &'a str,
    pub attach2: &'a str,
    pub constraintsystem: &'a str,
    pub forcelimit: f32,
    pub teleportfollowdistance: f32,
    pub torquelimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysBoneFollower<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysConstraint<'a> {
    pub attach1: &'a str,
    pub attach2: &'a str,
    pub constraintsystem: &'a str,
    pub forcelimit: f32,
    pub teleportfollowdistance: f32,
    pub torquelimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysConstraintsystem {
    pub additionaliterations: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysConvert<'a> {
    pub massoverride: f32,
    pub swapmodel: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysHinge<'a> {
    pub attach1: &'a str,
    pub attach2: &'a str,
    pub constraintsystem: &'a str,
    pub forcelimit: f32,
    pub hingeaxis: Vector,
    pub hingefriction: f32,
    pub maxSoundThreshold: f32,
    pub minSoundThreshold: f32,
    pub reversalsoundthresholdLarge: f32,
    pub reversalsoundthresholdMedium: f32,
    pub reversalsoundthresholdSmall: f32,
    pub systemloadscale: f32,
    pub teleportfollowdistance: f32,
    pub torquelimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysKeepupright<'a> {
    pub angularlimit: f32,
    pub attach1: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysLengthconstraint<'a> {
    pub addlength: f32,
    pub attach1: &'a str,
    pub attach2: &'a str,
    pub constraintsystem: &'a str,
    pub forcelimit: f32,
    pub minlength: f32,
    pub teleportfollowdistance: f32,
    pub torquelimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysMagnet<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub forcelimit: f32,
    pub hitboxset: i32,
    pub massScale: f32,
    pub maxobjects: i32,
    pub modelscale: f32,
    pub overridescript: &'a str,
    pub playbackrate: f32,
    pub sequence: i32,
    pub torquelimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysMotor<'a> {
    pub addangaccel: f32,
    pub attach1: &'a str,
    pub spinup: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysPulleyconstraint<'a> {
    pub addlength: f32,
    pub attach1: &'a str,
    pub attach2: &'a str,
    pub constraintsystem: &'a str,
    pub forcelimit: f32,
    pub gearratio: f32,
    pub teleportfollowdistance: f32,
    pub torquelimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysRagdollconstraint<'a> {
    pub attach1: &'a str,
    pub attach2: &'a str,
    pub constraintsystem: &'a str,
    pub forcelimit: f32,
    pub teleportfollowdistance: f32,
    pub torquelimit: f32,
    pub xfriction: f32,
    pub xmax: f32,
    pub xmin: f32,
    pub yfriction: f32,
    pub ymax: f32,
    pub ymin: f32,
    pub zfriction: f32,
    pub zmax: f32,
    pub zmin: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysRagdollmagnet {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub axis: Vector,
    pub force: f32,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysSlideconstraint<'a> {
    pub attach1: &'a str,
    pub attach2: &'a str,
    pub constraintsystem: &'a str,
    pub forcelimit: f32,
    pub maxSoundThreshold: f32,
    pub minSoundThreshold: f32,
    pub reversalsoundthresholdLarge: f32,
    pub reversalsoundthresholdMedium: f32,
    pub reversalsoundthresholdSmall: f32,
    pub slidefriction: f32,
    pub systemloadscale: f32,
    pub teleportfollowdistance: f32,
    pub torquelimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysSpring<'a> {
    pub angles: Angles,
    pub attach1: &'a str,
    pub attach2: &'a str,
    pub constant: f32,
    pub damping: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub length: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub relativedamping: f32,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysThruster<'a> {
    pub attach1: &'a str,
    pub force: f32,
    pub forcetime: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysTorque<'a> {
    pub attach1: &'a str,
    pub axis: Vector,
    pub force: f32,
    pub forcetime: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysicsCannister<'a> {
    pub Relationship: &'a str,
    pub expdamage: f32,
    pub expradius: f32,
    pub fuel: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysicsEntitySolver {}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysicsNpcSolver {}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysicsProp<'a> {
    pub BreakModelMessage: &'a str,
    pub Damagetype: i32,
    pub ExplodeDamage: f32,
    pub ExplodeRadius: f32,
    pub PerformanceMode: i32,
    pub PressureDelay: f32,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub minhealthdmg: i32,
    pub overridescript: &'a str,
    pub puntsound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysicsPropRagdoll<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angleOverride: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Plasma {}
#[derive(Debug, Clone, Deserialize)]
pub struct Player {}
#[derive(Debug, Clone, Deserialize)]
pub struct Player {}
#[derive(Debug, Clone, Deserialize)]
pub struct Player {}
#[derive(Debug, Clone, Deserialize)]
pub struct Player {}
#[derive(Debug, Clone, Deserialize)]
pub struct Player {}
#[derive(Debug, Clone, Deserialize)]
pub struct Player {}
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerLoadsaved<'a> {
    pub duration: f32,
    pub holdtime: f32,
    pub loadtime: f32,
    pub message: &'a str,
    pub messagetime: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerManager<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerPickup<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerPickup<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerSpeedmod {}
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerWeaponstrip {}
#[derive(Debug, Clone, Deserialize)]
pub struct PointAnglesensor<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub duration: f32,
    pub lookatname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointAngularvelocitysensor {
    pub axis: Vector,
    pub fireinterval: f32,
    pub threshold: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub usehelper: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointAntlionRepellant {
    pub repelradius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointApcController<'a> {
    pub maxRange: f32,
    pub minRange: f32,
    pub pitchrate: f32,
    pub pitchtolerance: f32,
    pub targetentityname: &'a str,
    pub yawrate: f32,
    pub yawtolerance: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointBonusmapsAccessor<'a> {
    pub filename: &'a str,
    pub mapname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointBugbait {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Enabled: bool,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointCamera<'a> {
    pub FOV: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub UseScreenAspectRatio: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub fogColor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub fogEnable: bool,
    pub fogEnd: f32,
    pub fogMaxDensity: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub fogRadial: bool,
    pub fogStart: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub resolution: f32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointClientcommand {}
#[derive(Debug, Clone, Deserialize)]
pub struct PointCombineBallLauncher<'a> {
    pub ballcount: i32,
    pub ballradius: f32,
    pub ballrespawntime: f32,
    pub bullseyename: &'a str,
    pub launchconenoise: f32,
    pub maxballbounces: i32,
    pub maxspeed: f32,
    pub minspeed: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointCommentaryNode<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub commentaryfile: &'a str,
    pub commentaryfile_nohdr: &'a str,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub postcommands: &'a str,
    pub precommands: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub prevent_movement: bool,
    pub sequence: i32,
    pub speakers: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub start_disabled: bool,
    pub teleport_origin: Vector,
    pub viewposition: &'a str,
    pub viewtarget: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointCommentaryViewpoint {
    pub GlowProxySize: f32,
    pub HDRColorScale: f32,
    pub frame: f32,
    pub framerate: f32,
    pub scale: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointDevshotCamera<'a> {
    pub FOV: i32,
    pub angles: Angles,
    pub cameraname: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointEnableMotionFixup<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointFleshEffectTarget {
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointGamestatsCounter<'a> {
    pub Name: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointGamestatsCounter<'a> {
    pub Name: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointHurt<'a> {
    pub Damage: i32,
    pub DamageDelay: f32,
    pub DamageRadius: f32,
    pub DamageTarget: &'a str,
    pub DamageType: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointIntermission {}
#[derive(Debug, Clone, Deserialize)]
pub struct PointMessage<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub developeronly: bool,
    pub message: &'a str,
    pub radius: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointPlayermoveconstraint<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub radius: f32,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub speedfactor: f32,
    pub targetname: &'a str,
    pub width: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointPopulatorInterface {}
#[derive(Debug, Clone, Deserialize)]
pub struct PointPosecontroller<'a> {
    pub CycleFrequency: f32,
    pub FModAmplitude: f32,
    pub FModRate: f32,
    pub FModTimeOffset: f32,
    pub FModType: i32,
    pub InterpolationTime: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub InterpolationWrap: bool,
    pub PoseParameterName: &'a str,
    pub PoseValue: f32,
    pub PropName: &'a str,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointProximitySensor {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointPush {
    #[serde(deserialize_with = "deserialize_bool")]
    pub enabled: bool,
    pub inner_radius: f32,
    pub magnitude: f32,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointScriptTemplate<'a> {
    pub Template01: &'a str,
    pub Template02: &'a str,
    pub Template03: &'a str,
    pub Template04: &'a str,
    pub Template05: &'a str,
    pub Template06: &'a str,
    pub Template07: &'a str,
    pub Template08: &'a str,
    pub Template09: &'a str,
    pub Template10: &'a str,
    pub Template11: &'a str,
    pub Template12: &'a str,
    pub Template13: &'a str,
    pub Template14: &'a str,
    pub Template15: &'a str,
    pub Template16: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointServercommand {}
#[derive(Debug, Clone, Deserialize)]
pub struct PointSpotlight {
    pub HDRColorScale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub IgnoreSolid: bool,
    pub SpotlightLength: f32,
    pub SpotlightWidth: f32,
    pub mindxlevel: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointTeleport<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointTemplate<'a> {
    pub Template01: &'a str,
    pub Template02: &'a str,
    pub Template03: &'a str,
    pub Template04: &'a str,
    pub Template05: &'a str,
    pub Template06: &'a str,
    pub Template07: &'a str,
    pub Template08: &'a str,
    pub Template09: &'a str,
    pub Template10: &'a str,
    pub Template11: &'a str,
    pub Template12: &'a str,
    pub Template13: &'a str,
    pub Template14: &'a str,
    pub Template15: &'a str,
    pub Template16: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointTesla<'a> {
    pub angles: Angles,
    pub beamcount_max: i32,
    pub beamcount_min: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub interval_max: f32,
    pub interval_min: f32,
    pub lifetime_max: f32,
    pub lifetime_min: f32,
    pub m_Color: Color,
    pub m_SoundName: &'a str,
    pub m_SourceEntityName: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub m_bOn: bool,
    pub m_flRadius: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
    pub texture: &'a str,
    pub thick_max: f32,
    pub thick_min: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointVelocitysensor {
    pub axis: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub enabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointViewcontrol<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub interpolatepositiontoplayer: bool,
    pub maxs: Vector,
    pub mins: Vector,
    pub moveto: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetattachment: &'a str,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointWorldtext<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub font: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub orientation: i32,
    pub origin: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rainbow: bool,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
    pub textsize: f32,
    pub textspacingX: f32,
    pub textspacingY: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointWorldtext {}
#[derive(Debug, Clone, Deserialize)]
pub struct PopulatorInternalSpawnPoint {}
#[derive(Debug, Clone, Deserialize)]
pub struct PortableThumper<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PredictedViewmodel {}
#[derive(Debug, Clone, Deserialize)]
pub struct PropCombineBall<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropCoreball<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDoorRotating<'a> {
    pub ajarangles: Vector,
    pub axis: Vector,
    pub distance: f32,
    pub dmg: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub forceclosed: bool,
    pub hardware: i32,
    pub opendir: i32,
    pub returndelay: f32,
    pub slavename: &'a str,
    pub spawnpos: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDropshipContainer<'a> {
    pub Damagetype: i32,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub overridescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamic<'a> {
    pub BreakModelMessage: &'a str,
    pub DefaultAnim: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DisableBoneFollowers: bool,
    pub ExplodeDamage: f32,
    pub ExplodeRadius: f32,
    pub MaxAnimTime: f32,
    pub MinAnimTime: f32,
    pub PerformanceMode: i32,
    pub PressureDelay: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub RandomAnimation: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub minhealthdmg: i32,
    pub puntsound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamicOrnament<'a> {
    pub DefaultAnim: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DisableBoneFollowers: bool,
    pub InitialOwner: &'a str,
    pub MaxAnimTime: f32,
    pub MinAnimTime: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub RandomAnimation: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamicOverride<'a> {
    pub BreakModelMessage: &'a str,
    pub DefaultAnim: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DisableBoneFollowers: bool,
    pub ExplodeDamage: f32,
    pub ExplodeRadius: f32,
    pub MaxAnimTime: f32,
    pub MinAnimTime: f32,
    pub PerformanceMode: i32,
    pub PressureDelay: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub RandomAnimation: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub minhealthdmg: i32,
    pub puntsound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysics<'a> {
    pub BreakModelMessage: &'a str,
    pub Damagetype: i32,
    pub ExplodeDamage: f32,
    pub ExplodeRadius: f32,
    pub PerformanceMode: i32,
    pub PressureDelay: f32,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub minhealthdmg: i32,
    pub overridescript: &'a str,
    pub puntsound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsMultiplayer<'a> {
    pub Damagetype: i32,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub overridescript: &'a str,
    pub physicsmode: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsOverride<'a> {
    pub BreakModelMessage: &'a str,
    pub Damagetype: i32,
    pub ExplodeDamage: f32,
    pub ExplodeRadius: f32,
    pub PerformanceMode: i32,
    pub PressureDelay: f32,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub minhealthdmg: i32,
    pub overridescript: &'a str,
    pub puntsound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsRespawnable<'a> {
    pub Damagetype: i32,
    pub RespawnTime: f32,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub overridescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropRagdoll<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angleOverride: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropRagdollAttached<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angleOverride: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropScalable<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropSoccerBall<'a> {
    pub Damagetype: i32,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub overridescript: &'a str,
    pub trigger_name: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropSphere<'a> {
    pub Damagetype: i32,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub overridescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropStickybomb<'a> {
    pub Damagetype: i32,
    pub damagetoenablemotion: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub dud: bool,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub overridescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropThumper<'a> {
    pub EffectRadius: i32,
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub dustscale: i32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehicle<'a> {
    pub VehicleScript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehicleAirboat {
    #[serde(deserialize_with = "deserialize_bool")]
    pub VehicleLocked: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehicleApc<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub VehicleLocked: bool,
    pub missilehint: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehicleCannon<'a> {
    pub vehiclescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehicleChoreoGeneric<'a> {
    pub DefaultAnim: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DisableBoneFollowers: bool,
    pub MaxAnimTime: f32,
    pub MinAnimTime: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub RandomAnimation: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoremoveparent: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreplayer: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub useplayereyes: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub vehiclelocked: bool,
    pub vehiclescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehicleCrane<'a> {
    pub magnetname: &'a str,
    pub vehiclescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehicleDriveable<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub VehicleLocked: bool,
    pub VehicleScript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehicleJeep {
    #[serde(deserialize_with = "deserialize_bool")]
    pub CargoVisible: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehicleJeep {
    #[serde(deserialize_with = "deserialize_bool")]
    pub VehicleLocked: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehicleJeep {
    #[serde(deserialize_with = "deserialize_bool")]
    pub VehicleLocked: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehicleJetski {
    #[serde(deserialize_with = "deserialize_bool")]
    pub VehicleLocked: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropVehiclePrisonerPod<'a> {
    pub Damagetype: i32,
    pub damagetoenablemotion: i32,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub overridescript: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub vehiclelocked: bool,
    pub vehiclescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ProtoSniper<'a> {
    pub PaintInterval: f32,
    pub PaintIntervalVariance: f32,
    pub additionalequipment: &'a str,
    pub beambrightness: i32,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub misses: i32,
    pub shielddistance: f32,
    pub shieldradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub shootZombiesInChest: bool,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Raggib<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct RdRobotDispenser {}
#[derive(Debug, Clone, Deserialize)]
pub struct ReservedSpot<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct RopeAnchor {}
#[derive(Debug, Clone, Deserialize)]
pub struct RpgMissile<'a> {
    pub Relationship: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct RpgMissile<'a> {
    pub Relationship: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SceneManager<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ScriptIntro<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub alternatefovchange: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ScriptedScene<'a> {
    pub ResumeSceneFile: &'a str,
    pub SceneFile: &'a str,
    pub busyactor: i32,
    pub onplayerdeath: i32,
    pub target1: &'a str,
    pub target2: &'a str,
    pub target3: &'a str,
    pub target4: &'a str,
    pub target5: &'a str,
    pub target6: &'a str,
    pub target7: &'a str,
    pub target8: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ScriptedSentence<'a> {
    pub attenuation: i32,
    pub delay: f32,
    pub entity: &'a str,
    pub listener: &'a str,
    pub radius: f32,
    pub refire: f32,
    pub sentence: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ScriptedSequence<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub m_bDisableNPCCollisions: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub m_bIgnoreGravity: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub m_bLoopActionSequence: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub m_bSynchPostIdles: bool,
    pub m_fMoveTo: i32,
    pub m_flRadius: f32,
    pub m_flRepeat: f32,
    pub m_iszCustomMove: &'a str,
    pub m_iszEntity: &'a str,
    pub m_iszEntry: &'a str,
    pub m_iszIdle: &'a str,
    pub m_iszNextScript: &'a str,
    pub m_iszPlay: &'a str,
    pub m_iszPostIdle: &'a str,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub onplayerdeath: i32,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ScriptedTarget<'a> {
    pub EffectDuration: f32,
    pub MoveSpeed: i32,
    pub PauseDuration: f32,
    pub StartDisabled: i32,
    pub additionalequipment: &'a str,
    pub enemyfilter: &'a str,
    pub hintgroup: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub hintlimiting: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ignoreunseenenemies: bool,
    pub m_flRadius: f32,
    pub m_iszEntity: &'a str,
    pub sleepstate: i32,
    pub squadname: &'a str,
    pub wakeradius: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub wakesquad: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SdkBot {}
#[derive(Debug, Clone, Deserialize)]
pub struct SdkGamerules {}
#[derive(Debug, Clone, Deserialize)]
pub struct SdkRagdoll {}
#[derive(Debug, Clone, Deserialize)]
pub struct SdkTeamManager {}
#[derive(Debug, Clone, Deserialize)]
pub struct ShadowControl<'a> {
    pub angles: Angles,
    pub color: Color,
    pub direction: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableallshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub distance: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SimpleBot {}
#[derive(Debug, Clone, Deserialize)]
pub struct SimplePhysicsBrush<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SimplePhysicsProp {}
#[derive(Debug, Clone, Deserialize)]
pub struct SkyCamera {
    #[serde(deserialize_with = "deserialize_bool")]
    pub fogblend: bool,
    pub fogcolor: Color,
    pub fogcolor2: Color,
    pub fogdir: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub fogenable: bool,
    pub fogend: f32,
    pub fogmaxdensity: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub fogradial: bool,
    pub fogstart: f32,
    pub scale: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub use_angles: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SkyboxSwapper<'a> {
    pub SkyboxName: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Sniperbullet<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Soundent {}
#[derive(Debug, Clone, Deserialize)]
pub struct SparkShower {}
#[derive(Debug, Clone, Deserialize)]
pub struct Sparktrail {}
#[derive(Debug, Clone, Deserialize)]
pub struct SpotlightEnd<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Spraycan {}
#[derive(Debug, Clone, Deserialize)]
pub struct Squadinsignia<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TankBoss<'a> {
    pub health: i32,
    pub model: &'a str,
    pub speed: f32,
    pub start_disabled: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TankBoss {}
#[derive(Debug, Clone, Deserialize)]
pub struct TankDestruction<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TanktrainAi<'a> {
    pub enginesound: &'a str,
    pub movementsound: &'a str,
    pub startsound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TanktrainAitarget<'a> {
    pub newtarget: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TargetCdaudio {
    pub track: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TargetChangegravity {
    pub gravity: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeTester {}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamControlPoint<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub point_capture_end_sound: &'a str,
    pub point_capture_interrupted_sound: &'a str,
    pub point_capture_progress_sound: &'a str,
    pub point_capture_start_sound: &'a str,
    pub point_default_owner: i32,
    pub point_group: i32,
    pub point_index: i32,
    pub point_printname: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub point_start_locked: bool,
    pub point_warn_on_cap: i32,
    pub point_warn_sound: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub random_owner_on_restart: bool,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamControlPointMaster<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    pub caplayout: &'a str,
    pub cpm_restrict_team_cap_win: i32,
    pub custom_position_x: f32,
    pub custom_position_y: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub partial_cap_points_rate: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub play_all_rounds: bool,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub score_style: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub switch_teams: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamControlPointRound<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    pub cpr_cp_names: &'a str,
    pub cpr_printname: &'a str,
    pub cpr_priority: i32,
    pub cpr_restrict_team_cap_win: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamManager<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamRoundTimer<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub auto_countdown: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub max_length: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub reset_time: bool,
    pub setup_length: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub show_in_hud: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub show_time_remaining: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub start_paused: bool,
    pub targetname: &'a str,
    pub timer_length: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamTrainWatcher<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub env_spark_name: &'a str,
    pub goal_node: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub handle_train_movement: bool,
    pub hud_min_speed_level_1: f32,
    pub hud_min_speed_level_2: f32,
    pub hud_min_speed_level_3: f32,
    pub linked_cp_1: &'a str,
    pub linked_cp_2: &'a str,
    pub linked_cp_3: &'a str,
    pub linked_cp_4: &'a str,
    pub linked_cp_5: &'a str,
    pub linked_cp_6: &'a str,
    pub linked_cp_7: &'a str,
    pub linked_cp_8: &'a str,
    pub linked_pathtrack_1: &'a str,
    pub linked_pathtrack_2: &'a str,
    pub linked_pathtrack_3: &'a str,
    pub linked_pathtrack_4: &'a str,
    pub linked_pathtrack_5: &'a str,
    pub linked_pathtrack_6: &'a str,
    pub linked_pathtrack_7: &'a str,
    pub linked_pathtrack_8: &'a str,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub speed_forward_modifier: f32,
    pub start_node: &'a str,
    pub targetname: &'a str,
    pub train: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub train_can_recede: bool,
    pub train_recede_time: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TeamTrainWatcherMaster {}
#[derive(Debug, Clone, Deserialize)]
pub struct TeleportVortex<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TestEffect<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TestProxytoggle<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TestTraceline {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfAmmoPack {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfBaseMinigame<'a> {
    pub BlueSpawn: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub InRandomPool: bool,
    pub MaxScore: i32,
    pub RedSpawn: &'a str,
    pub ScoreType: i32,
    pub SuddenDeathTime: f32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub enemy_team_score_sound: &'a str,
    pub hud_res_file: &'a str,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
    pub your_team_score_sound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfBonusDuckPickup<'a> {
    pub pickup_particle: &'a str,
    pub pickup_sound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfBot {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfDroppedWeapon<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfFlame<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfFlameManager {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfGamerules {
    #[serde(deserialize_with = "deserialize_bool")]
    pub ctf_overtime: bool,
    pub hud_type: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub ropes_holiday_lights_allowed: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfGasGrenadeEffect<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfGasManager {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfGenericBomb<'a> {
    pub damage: f32,
    pub explode_particle: &'a str,
    pub friendlyfire: i32,
    pub health: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub passActivator: bool,
    pub radius: f32,
    pub sound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfGlow<'a> {
    pub GlowColor: Color,
    pub Mode: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfHalloweenGiftPickup<'a> {
    pub pickup_particle: &'a str,
    pub pickup_sound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfHalloweenGiftSpawnLocation<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfHalloweenItemPickup {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfHalloweenMinigame<'a> {
    pub BlueSpawn: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub InRandomPool: bool,
    pub MaxScore: i32,
    pub MinigameType: i32,
    pub RedSpawn: &'a str,
    pub ScoreType: i32,
    pub SuddenDeathTime: f32,
    pub enemy_team_score_sound: &'a str,
    pub hud_res_file: &'a str,
    pub your_team_score_sound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfHalloweenMinigameFallingPlatforms {
    pub MinigameType: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfHalloweenPickup<'a> {
    pub pickup_particle: &'a str,
    pub pickup_sound: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicArena {
    pub CapEnableDelay: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicBonusround<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicBossBattle {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicCompetitive {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicCpTimer<'a> {
    pub controlpoint: &'a str,
    pub team_number: i32,
    pub timer_length: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicHoliday {
    pub allowHaunting: i32,
    pub holiday_type: i32,
    pub tauntInHell: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicHybridCtfCp {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicKoth {
    pub timer_length: i32,
    pub unlock_point: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicMannVsMachine {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicMannpower {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicMedieval {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicMinigames<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicMultipleEscort {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicOnHoliday {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicPlayerDestruction<'a> {
    pub finale_length: f32,
    pub flag_reset_delay: i32,
    pub heal_distance: i32,
    pub loser_respawn_bonus_per_bot: f32,
    pub max_points: i32,
    pub min_points: i32,
    pub points_per_player: i32,
    pub prop_drop_sound: &'a str,
    pub prop_model_name: &'a str,
    pub prop_pickup_sound: &'a str,
    pub res_file: &'a str,
    pub score_interval: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicRaid {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicRobotDestruction<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub finale_length: f32,
    pub loser_respawn_bonus_per_bot: f32,
    pub max_points: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub res_file: &'a str,
    pub score_interval: f32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfLogicTrainingMode<'a> {
    pub nextMap: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfMannVsMachineStats<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfMerasmusTrickOrTreatProp<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfObjectiveResource {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfPdaExpansionDispenser {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfPdaExpansionTeleporter {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfPlayerManager {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfPointNavInterface {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfPointWeaponMimic<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub Crits: bool,
    pub Damage: f32,
    pub ModelScale: f32,
    pub ParticleEffect: &'a str,
    pub SpeedMax: f32,
    pub SpeedMin: f32,
    pub SplashRadius: f32,
    pub SpreadAngle: f32,
    pub WeaponType: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfPowerupBottle {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileArrow {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileBallOrnament {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileBalloffire {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileCleaver {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileEnergyBall {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileEnergyRing {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileFlare {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileGrapplinghook {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileHealingBolt {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileJar {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileJarGas {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileJarMilk {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileLightningorb {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileMechanicalarmorb {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectilePipe {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectilePipeRemote {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileRocket {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSentryrocket {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellbats {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellfireball {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellkartbats {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellkartmirv {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellkartorb {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellkartpumpkin {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellmeteorshower {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellmirv {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellpumpkin {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellspawnboss {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellspawnhorde {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpellspawnzombie {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSpelltransposeteleport {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileStunBall {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileSyringe {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileThrowable {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileThrowableBreadmonster {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileThrowableBrick {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfProjectileThrowableRepel {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfPumpkinBomb {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfRagdoll {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfRobotDestructionRobot {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfRobotDestructionRobotSpawn<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub gibs: i32,
    pub health: i32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub points: i32,
    pub r#type: i32,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub spawngroup: &'a str,
    pub startpath: &'a str,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfRobotDestructionSpawnGroup<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub group_number: i32,
    pub hud_icon: &'a str,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub respawn_reduction_scale: f32,
    pub respawn_time: f32,
    pub targetname: &'a str,
    pub team_number: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfSpawner<'a> {
    pub count: i32,
    pub interval: f32,
    pub maxActive: i32,
    pub template: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfSpawnerBoss<'a> {
    pub count: i32,
    pub interval: f32,
    pub maxActive: i32,
    pub team: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfSpellMeteorshowerspawner<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfSpellPickup<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub AutoMaterialize: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub powerup_model: &'a str,
    pub tier: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfTargetDummy<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfTauntProp<'a> {
    pub Relationship: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TfTeam {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfTeleportLocation {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfTemplateStunDrone {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfViewmodel {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponBase {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponBat {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponBatFish {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponBatGiftwrap {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponBatWood {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponBonesaw {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponBoomerang {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponBottle {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponBreakableSign {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponBuffItem {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponBuilder {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponChargedSmg {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponCleaver {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponClub {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponCompoundBow {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponCrossbow {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponCrowbar {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponDecoy {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponDrgPomson {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponDrgPomson {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponFireaxe {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponFists {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponFlag {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponFlamethrower {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponFlaregun {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponFlaregunRevenge {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrapplinghook {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeCaltrop {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeCaltropProjectile {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeConcussion {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeConcussionProjectile {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeEmp {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeEmpProjectile {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeGas {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeGasProjectile {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeHeal {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeHealProjectile {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeMirv {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeMirvBomb {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeMirvDemoman {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeMirvProjectile {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeNail {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeNailProjectile {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeNapalm {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeNapalmProjectile {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeNormal {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeNormalProjectile {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadeSmokeBomb {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponGrenadelauncher {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponHandgunScoutPrimary {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponHandgunScoutSecondary {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponInvis {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponJar {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponJarGas {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponJarMilk {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponKatana {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponKnife {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponLaserPointer {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponLunchbox {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponLunchboxDrink {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponMechanicalArm {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponMedigun {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponMinigun {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponNailgun {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponParticleCannon {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponPasstimeGun {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponPdaEngineerBuild {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponPdaEngineerDestroy {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponPdaSpy {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponPipebomblauncher {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponPistol {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponPistolScout {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponRaygun {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponRaygun {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponRevolver {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponRobotArm {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponRocketlauncher {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponRocketlauncherAirstrike {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponRocketlauncherDirecthit {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponRocketlauncherFireball {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponRocketpack {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponSapper {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponShovel {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponSlap {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponSniperrifle {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponSniperrifleClassic {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponSniperrifleDecap {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponSpellbook {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponStickbomb {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponSword {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponSyringegunMedic {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponThrowable {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponTranq {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponWrench {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponbaseGrenade {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponbaseGrenadeProj {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponbaseMelee {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWeaponbaseMerasmusGrenade {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWearable {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWearableCampaignItem {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWearableDemoshield {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWearableLevelableItem {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWearableRazorback {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWearableRobotArm {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfWearableVm {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfZombie {}
#[derive(Debug, Clone, Deserialize)]
pub struct TfZombieSpawner {
    #[serde(deserialize_with = "deserialize_bool")]
    pub infinite_zombies: bool,
    pub max_zombies: i32,
    pub zombie_lifetime: f32,
    pub zombie_type: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TrainingAnnotation<'a> {
    pub display_text: &'a str,
    pub lifetime: f32,
    pub offset: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TrainingPropDynamic<'a> {
    pub DefaultAnim: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DisableBoneFollowers: bool,
    pub MaxAnimTime: f32,
    pub MinAnimTime: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub RandomAnimation: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Trigger<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
    pub master: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerAddOrRemoveTfPlayerAttributes<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub add_or_remove: bool,
    pub attribute_name: &'a str,
    pub duration: f32,
    pub filtername: &'a str,
    pub value: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerAddTfPlayerCondition<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub condition: i32,
    pub duration: f32,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerApplyImpulse<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
    pub force: f32,
    pub impulse_dir: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerAutosave<'a> {
    pub DangerousTimer: f32,
    pub MinimumHitPoints: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub NewLevelUnit: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerBotTag<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub add: bool,
    pub filtername: &'a str,
    pub tags: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerBrush<'a> {
    pub DontMessageParent: i32,
    pub InputFilter: i32,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerCaptureArea<'a> {
    pub area_cap_point: &'a str,
    pub area_time_to_cap: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerCatapult<'a> {
    pub AirCtrlSupressionTime: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub DirectionSuppressAirControl: bool,
    pub EntryAngleTolerance: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub applyAngularImpulse: bool,
    pub exactVelocityChoiceType: i32,
    pub filtername: &'a str,
    pub launchDirection: Vector,
    pub launchTarget: &'a str,
    pub lowerThreshold: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub onlyVelocityCheck: bool,
    pub physicsSpeed: f32,
    pub playerSpeed: f32,
    pub upperThreshold: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub useExactVelocity: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub useThresholdCheck: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerCdaudio<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerChangelevel<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerGravity<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerHurt {
    pub damage: f32,
    pub damagecap: f32,
    pub damagemodel: i32,
    pub damagetype: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodmgforce: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerIgnite<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub burn_duration: f32,
    pub damage_percent_per_second: f32,
    pub filtername: &'a str,
    pub ignite_particle_name: &'a str,
    pub ignite_sound_name: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerIgniteArrows<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerImpact {
    pub Magnitude: f32,
    pub Noise: f32,
    pub Viewkick: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerLook {
    pub timeout: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerMultiple<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerOnce {}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerParticle<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub attachment_name: &'a str,
    pub attachment_type: i32,
    pub filtername: &'a str,
    pub particle_name: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPasstimeBall<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPhysicsTrap {
    pub dissolvetype: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPlayerRespawnOverride<'a> {
    pub RespawnName: &'a str,
    pub RespawnTime: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPlayermovement<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPortal<'a> {
    pub RemotePortal: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerProximity<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
    pub measuretarget: &'a str,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPush<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub alternateticksfix: f32,
    pub filtername: &'a str,
    pub pushdir: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerRdVaultTrigger<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerRemove<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerRemoveTfPlayerCondition<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub condition: i32,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerRpgfire {}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerServerragdoll<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerSoundscape<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
    pub soundscape: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerStun<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
    pub move_speed_reduction: f32,
    pub stun_duration: f32,
    pub stun_effects: i32,
    pub stun_type: i32,
    pub trigger_delay: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerSuperArmor {}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTeleport<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
    pub landmark: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTeleportRelative<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
    pub teleportoffset: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTimerDoor<'a> {
    pub area_cap_point: &'a str,
    pub area_time_to_cap: f32,
    pub door_name: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTogglesave<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTransition {}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerVehicleCargo<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerVphysicsMotion<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerWaterydeath<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerWeaponDissolve<'a> {
    pub emittername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerWeaponStrip {
    #[serde(deserialize_with = "deserialize_bool")]
    pub KillWeapons: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerWind<'a> {
    pub DirectionNoise: i32,
    pub HoldNoise: i32,
    pub HoldTime: i32,
    pub SpeedNoise: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub StartDisabled: bool,
    pub filtername: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TripwireHook<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct VehicleViewcontroller {
    #[serde(deserialize_with = "deserialize_bool")]
    pub VehicleLocked: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct VguiScreen<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub height: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub overlaymaterial: &'a str,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
    pub width: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct VguiScreenTeam<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub height: f32,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub overlaymaterial: &'a str,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
    pub width: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct VguiSlideshowDisplay<'a> {
    pub angles: Angles,
    pub cycletype: i32,
    pub directory: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub height: i32,
    pub maxs: Vector,
    pub maxslidetime: f32,
    pub mins: Vector,
    pub minslidetime: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nolistrepeats: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
    pub width: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Viewangleanim {}
#[derive(Debug, Clone, Deserialize)]
pub struct Viewmodel<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct VortChargeToken<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct VortEffectDispel<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct VortexController<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct VoteController<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WaterLodControl<'a> {
    pub angles: Angles,
    pub cheapwaterenddistance: f32,
    pub cheapwaterstartdistance: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub maxs: Vector,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Waterbullet<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Weapon357 {}
#[derive(Debug, Clone, Deserialize)]
pub struct Weapon357 {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAlyxgun {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAnnabelle {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAr1 {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAr2 {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAr2 {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponBrickbat {
    pub BrickbatType: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponBugbait {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponCguard {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponCitizenpackage {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponCitizensuitcase {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponCrossbow {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponCrossbow {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponCrowbar {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponCrowbar {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponCrowbar {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponCubemap {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponExtinguisher {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFlaregun {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFrag {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFrag {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponGrenade {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponHl2mpBase {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponHopwire {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponIfmBase {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponIfmBaseCamera {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponIfmSteadycam {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponImmolator {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponManhack {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponMolotov {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponMp5 {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponOldmanharpoon {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponPhyscannon {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponPhyscannon {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponPhysgun {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponPistol {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponPistol {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponPistol {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponProto1 {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponRpg {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponRpg {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSdkBase {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponShotgun {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponShotgun {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponShotgun {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSlam {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSlam {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSmg1 {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSmg1 {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSmg2 {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSniperrifle {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponStriderbuster<'a> {
    pub Damagetype: i32,
    pub damagetoenablemotion: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub dud: bool,
    pub forcetoenablemotion: f32,
    pub inertiascale: f32,
    pub massscale: f32,
    pub overridescript: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponStunstick {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponStunstick {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponThumper {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponThumper {}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponTripwire {}
#[derive(Debug, Clone, Deserialize)]
pub struct WearableItem {}
#[derive(Debug, Clone, Deserialize)]
pub struct WheelOfDoom<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub effect_duration: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub has_spiral: bool,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WheelOfDoomSpiral<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WindowPane<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WorldItems<'a> {
    pub LightingOrigin: &'a str,
    pub LightingOriginHack: &'a str,
    pub body: i32,
    pub cycle: f32,
    pub fadescale: f32,
    pub hitboxset: i32,
    pub modelscale: f32,
    pub playbackrate: f32,
    pub r#type: i32,
    pub sequence: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Worldspawn<'a> {
    pub angles: Angles,
    pub chaptertitle: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub coldworld: bool,
    pub detailmaterial: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub gametitle: bool,
    pub maxoccludeearea: f32,
    pub maxoccludeearea_x360: f32,
    pub maxpropscreenwidth: f32,
    pub maxs: Vector,
    pub minoccluderarea: f32,
    pub minoccluderarea_x360: f32,
    pub minpropscreenwidth: f32,
    pub mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdark: bool,
    pub targetname: &'a str,
}
