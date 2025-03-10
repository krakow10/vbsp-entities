use serde::Deserialize;
use vbsp_common::deserialize_bool;
use vbsp_common::{Angles, Color, LightColor, Negated, Vector};
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
#[serde(tag = "classname")]
pub enum Entity<'a> {
    #[serde(rename = "ambient_generic")]
    #[serde(borrow)]
    AmbientGeneric(AmbientGeneric<'a>),
    #[serde(rename = "cycler")]
    #[serde(borrow)]
    Cycler(Cycler<'a>),
    #[serde(rename = "env_beam")]
    #[serde(borrow)]
    EnvBeam(EnvBeam<'a>),
    #[serde(rename = "env_bubbles")]
    #[serde(borrow)]
    EnvBubbles(EnvBubbles<'a>),
    #[serde(rename = "env_detail_controller")]
    EnvDetailController(EnvDetailController),
    #[serde(rename = "env_embers")]
    #[serde(borrow)]
    EnvEmbers(EnvEmbers<'a>),
    #[serde(rename = "env_entity_maker")]
    #[serde(borrow)]
    EnvEntityMaker(EnvEntityMaker<'a>),
    #[serde(rename = "env_explosion")]
    #[serde(borrow)]
    EnvExplosion(EnvExplosion<'a>),
    #[serde(rename = "env_fade")]
    #[serde(borrow)]
    EnvFade(EnvFade<'a>),
    #[serde(rename = "env_fire")]
    #[serde(borrow)]
    EnvFire(EnvFire<'a>),
    #[serde(rename = "env_fire_trail")]
    #[serde(borrow)]
    EnvFireTrail(EnvFireTrail<'a>),
    #[serde(rename = "env_firesource")]
    EnvFiresource(EnvFiresource),
    #[serde(rename = "env_fog_controller")]
    #[serde(borrow)]
    EnvFogController(EnvFogController<'a>),
    #[serde(rename = "env_hudhint")]
    #[serde(borrow)]
    EnvHudhint(EnvHudhint<'a>),
    #[serde(rename = "env_laser")]
    #[serde(borrow)]
    EnvLaser(EnvLaser<'a>),
    #[serde(rename = "env_lightglow")]
    EnvLightglow(EnvLightglow),
    #[serde(rename = "env_physexplosion")]
    #[serde(borrow)]
    EnvPhysexplosion(EnvPhysexplosion<'a>),
    #[serde(rename = "env_projectedtexture")]
    #[serde(borrow)]
    EnvProjectedtexture(EnvProjectedtexture<'a>),
    #[serde(rename = "env_screenoverlay")]
    #[serde(borrow)]
    EnvScreenoverlay(EnvScreenoverlay<'a>),
    #[serde(rename = "env_shake")]
    #[serde(borrow)]
    EnvShake(EnvShake<'a>),
    #[serde(rename = "env_shooter")]
    #[serde(borrow)]
    EnvShooter(EnvShooter<'a>),
    #[serde(rename = "env_smokestack")]
    #[serde(borrow)]
    EnvSmokestack(EnvSmokestack<'a>),
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
    #[serde(borrow)]
    EnvSpark(EnvSpark<'a>),
    #[serde(rename = "env_sprite")]
    #[serde(borrow)]
    EnvSprite(EnvSprite<'a>),
    #[serde(rename = "env_spritetrail")]
    #[serde(borrow)]
    EnvSpritetrail(EnvSpritetrail<'a>),
    #[serde(rename = "env_steam")]
    #[serde(borrow)]
    EnvSteam(EnvSteam<'a>),
    #[serde(rename = "env_sun")]
    #[serde(borrow)]
    EnvSun(EnvSun<'a>),
    #[serde(rename = "env_tonemap_controller")]
    #[serde(borrow)]
    EnvTonemapController(EnvTonemapController<'a>),
    #[serde(rename = "env_wind")]
    EnvWind(EnvWind),
    #[serde(rename = "filter_activator_class")]
    #[serde(borrow)]
    FilterActivatorClass(FilterActivatorClass<'a>),
    #[serde(rename = "filter_activator_name")]
    #[serde(borrow)]
    FilterActivatorName(FilterActivatorName<'a>),
    #[serde(rename = "filter_damage_type")]
    #[serde(borrow)]
    FilterDamageType(FilterDamageType<'a>),
    #[serde(rename = "filter_multi")]
    #[serde(borrow)]
    FilterMulti(FilterMulti<'a>),
    #[serde(rename = "func_areaportal")]
    #[serde(borrow)]
    FuncAreaportal(FuncAreaportal<'a>),
    #[serde(rename = "func_areaportalwindow")]
    #[serde(borrow)]
    FuncAreaportalwindow(FuncAreaportalwindow<'a>),
    #[serde(rename = "func_bomb_target")]
    #[serde(borrow)]
    FuncBombTarget(FuncBombTarget<'a>),
    #[serde(rename = "func_breakable")]
    #[serde(borrow)]
    FuncBreakable(FuncBreakable<'a>),
    #[serde(rename = "func_breakable_surf")]
    #[serde(borrow)]
    FuncBreakableSurf(FuncBreakableSurf<'a>),
    #[serde(rename = "func_brush")]
    #[serde(borrow)]
    FuncBrush(FuncBrush<'a>),
    #[serde(rename = "func_button")]
    #[serde(borrow)]
    FuncButton(FuncButton<'a>),
    #[serde(rename = "func_buyzone")]
    #[serde(borrow)]
    FuncBuyzone(FuncBuyzone<'a>),
    #[serde(rename = "func_clip_vphysics")]
    #[serde(borrow)]
    FuncClipVphysics(FuncClipVphysics<'a>),
    #[serde(rename = "func_conveyor")]
    #[serde(borrow)]
    FuncConveyor(FuncConveyor<'a>),
    #[serde(rename = "func_door")]
    #[serde(borrow)]
    FuncDoor(FuncDoor<'a>),
    #[serde(rename = "func_door_rotating")]
    #[serde(borrow)]
    FuncDoorRotating(FuncDoorRotating<'a>),
    #[serde(rename = "func_dustcloud")]
    #[serde(borrow)]
    FuncDustcloud(FuncDustcloud<'a>),
    #[serde(rename = "func_dustmotes")]
    #[serde(borrow)]
    FuncDustmotes(FuncDustmotes<'a>),
    #[serde(rename = "func_fish_pool")]
    #[serde(borrow)]
    FuncFishPool(FuncFishPool<'a>),
    #[serde(rename = "func_footstep_control")]
    #[serde(borrow)]
    FuncFootstepControl(FuncFootstepControl<'a>),
    #[serde(rename = "func_hostage_rescue")]
    #[serde(borrow)]
    FuncHostageRescue(FuncHostageRescue<'a>),
    #[serde(rename = "func_illusionary")]
    #[serde(borrow)]
    FuncIllusionary(FuncIllusionary<'a>),
    #[serde(rename = "func_lod")]
    #[serde(borrow)]
    FuncLod(FuncLod<'a>),
    #[serde(rename = "func_monitor")]
    #[serde(borrow)]
    FuncMonitor(FuncMonitor<'a>),
    #[serde(rename = "func_movelinear")]
    #[serde(borrow)]
    FuncMovelinear(FuncMovelinear<'a>),
    #[serde(rename = "func_occluder")]
    #[serde(borrow)]
    FuncOccluder(FuncOccluder<'a>),
    #[serde(rename = "func_physbox")]
    #[serde(borrow)]
    FuncPhysbox(FuncPhysbox<'a>),
    #[serde(rename = "func_physbox_multiplayer")]
    #[serde(borrow)]
    FuncPhysboxMultiplayer(FuncPhysboxMultiplayer<'a>),
    #[serde(rename = "func_precipitation")]
    #[serde(borrow)]
    FuncPrecipitation(FuncPrecipitation<'a>),
    #[serde(rename = "func_rot_button")]
    #[serde(borrow)]
    FuncRotButton(FuncRotButton<'a>),
    #[serde(rename = "func_rotating")]
    #[serde(borrow)]
    FuncRotating(FuncRotating<'a>),
    #[serde(rename = "func_smokevolume")]
    #[serde(borrow)]
    FuncSmokevolume(FuncSmokevolume<'a>),
    #[serde(rename = "func_tracktrain")]
    #[serde(borrow)]
    FuncTracktrain(FuncTracktrain<'a>),
    #[serde(rename = "func_train")]
    #[serde(borrow)]
    FuncTrain(FuncTrain<'a>),
    #[serde(rename = "func_wall")]
    #[serde(borrow)]
    FuncWall(FuncWall<'a>),
    #[serde(rename = "func_wall_toggle")]
    #[serde(borrow)]
    FuncWallToggle(FuncWallToggle<'a>),
    #[serde(rename = "func_water_analog")]
    #[serde(borrow)]
    FuncWaterAnalog(FuncWaterAnalog<'a>),
    #[serde(rename = "game_player_equip")]
    #[serde(borrow)]
    GamePlayerEquip(GamePlayerEquip<'a>),
    #[serde(rename = "game_text")]
    #[serde(borrow)]
    GameText(GameText<'a>),
    #[serde(rename = "game_ui")]
    #[serde(borrow)]
    GameUi(GameUi<'a>),
    #[serde(rename = "game_weapon_manager")]
    #[serde(borrow)]
    GameWeaponManager(GameWeaponManager<'a>),
    #[serde(rename = "hostage_entity")]
    #[serde(borrow)]
    HostageEntity(HostageEntity<'a>),
    #[serde(rename = "info_camera_link")]
    #[serde(borrow)]
    InfoCameraLink(InfoCameraLink<'a>),
    #[serde(rename = "info_ladder")]
    InfoLadder(InfoLadder),
    #[serde(rename = "info_lighting_relative")]
    #[serde(borrow)]
    InfoLightingRelative(InfoLightingRelative<'a>),
    #[serde(rename = "info_map_parameters")]
    InfoMapParameters(InfoMapParameters),
    #[serde(rename = "info_node")]
    InfoNode(InfoNode),
    #[serde(rename = "info_node_hint")]
    InfoNodeHint(InfoNodeHint),
    #[serde(rename = "info_particle_system")]
    #[serde(borrow)]
    InfoParticleSystem(InfoParticleSystem<'a>),
    #[serde(rename = "info_player_counterterrorist")]
    InfoPlayerCounterterrorist(InfoPlayerCounterterrorist),
    #[serde(rename = "info_player_logo")]
    InfoPlayerLogo(InfoPlayerLogo),
    #[serde(rename = "info_player_start")]
    InfoPlayerStart(InfoPlayerStart),
    #[serde(rename = "info_player_terrorist")]
    InfoPlayerTerrorist(InfoPlayerTerrorist),
    #[serde(rename = "info_target")]
    #[serde(borrow)]
    InfoTarget(InfoTarget<'a>),
    #[serde(rename = "info_teleport_destination")]
    #[serde(borrow)]
    InfoTeleportDestination(InfoTeleportDestination<'a>),
    #[serde(rename = "infodecal")]
    #[serde(borrow)]
    Infodecal(Infodecal<'a>),
    #[serde(rename = "keyframe_rope")]
    #[serde(borrow)]
    KeyframeRope(KeyframeRope<'a>),
    #[serde(rename = "light")]
    #[serde(borrow)]
    Light(Light<'a>),
    #[serde(rename = "light_environment")]
    #[serde(borrow)]
    LightEnvironment(LightEnvironment<'a>),
    #[serde(rename = "light_spot")]
    #[serde(borrow)]
    LightSpot(LightSpot<'a>),
    #[serde(rename = "logic_auto")]
    #[serde(borrow)]
    LogicAuto(LogicAuto<'a>),
    #[serde(rename = "logic_branch")]
    #[serde(borrow)]
    LogicBranch(LogicBranch<'a>),
    #[serde(rename = "logic_case")]
    #[serde(borrow)]
    LogicCase(LogicCase<'a>),
    #[serde(rename = "logic_compare")]
    #[serde(borrow)]
    LogicCompare(LogicCompare<'a>),
    #[serde(rename = "logic_measure_movement")]
    #[serde(borrow)]
    LogicMeasureMovement(LogicMeasureMovement<'a>),
    #[serde(rename = "logic_relay")]
    #[serde(borrow)]
    LogicRelay(LogicRelay<'a>),
    #[serde(rename = "logic_timer")]
    #[serde(borrow)]
    LogicTimer(LogicTimer<'a>),
    #[serde(rename = "math_counter")]
    #[serde(borrow)]
    MathCounter(MathCounter<'a>),
    #[serde(rename = "move_rope")]
    #[serde(borrow)]
    MoveRope(MoveRope<'a>),
    #[serde(rename = "path_track")]
    #[serde(borrow)]
    PathTrack(PathTrack<'a>),
    #[serde(rename = "phys_ballsocket")]
    #[serde(borrow)]
    PhysBallsocket(PhysBallsocket<'a>),
    #[serde(rename = "phys_constraint")]
    #[serde(borrow)]
    PhysConstraint(PhysConstraint<'a>),
    #[serde(rename = "phys_constraintsystem")]
    #[serde(borrow)]
    PhysConstraintsystem(PhysConstraintsystem<'a>),
    #[serde(rename = "phys_hinge")]
    #[serde(borrow)]
    PhysHinge(PhysHinge<'a>),
    #[serde(rename = "phys_keepupright")]
    #[serde(borrow)]
    PhysKeepupright(PhysKeepupright<'a>),
    #[serde(rename = "phys_lengthconstraint")]
    #[serde(borrow)]
    PhysLengthconstraint(PhysLengthconstraint<'a>),
    #[serde(rename = "phys_pulleyconstraint")]
    #[serde(borrow)]
    PhysPulleyconstraint(PhysPulleyconstraint<'a>),
    #[serde(rename = "phys_ragdollconstraint")]
    #[serde(borrow)]
    PhysRagdollconstraint(PhysRagdollconstraint<'a>),
    #[serde(rename = "phys_ragdollmagnet")]
    PhysRagdollmagnet(PhysRagdollmagnet),
    #[serde(rename = "phys_thruster")]
    #[serde(borrow)]
    PhysThruster(PhysThruster<'a>),
    #[serde(rename = "phys_torque")]
    #[serde(borrow)]
    PhysTorque(PhysTorque<'a>),
    #[serde(rename = "player_speedmod")]
    #[serde(borrow)]
    PlayerSpeedmod(PlayerSpeedmod<'a>),
    #[serde(rename = "player_weaponstrip")]
    #[serde(borrow)]
    PlayerWeaponstrip(PlayerWeaponstrip<'a>),
    #[serde(rename = "point_camera")]
    #[serde(borrow)]
    PointCamera(PointCamera<'a>),
    #[serde(rename = "point_clientcommand")]
    #[serde(borrow)]
    PointClientcommand(PointClientcommand<'a>),
    #[serde(rename = "point_devshot_camera")]
    #[serde(borrow)]
    PointDevshotCamera(PointDevshotCamera<'a>),
    #[serde(rename = "point_servercommand")]
    #[serde(borrow)]
    PointServercommand(PointServercommand<'a>),
    #[serde(rename = "point_spotlight")]
    #[serde(borrow)]
    PointSpotlight(PointSpotlight<'a>),
    #[serde(rename = "point_surroundtest")]
    #[serde(borrow)]
    PointSurroundtest(PointSurroundtest<'a>),
    #[serde(rename = "point_template")]
    #[serde(borrow)]
    PointTemplate(PointTemplate<'a>),
    #[serde(rename = "point_tesla")]
    #[serde(borrow)]
    PointTesla(PointTesla<'a>),
    #[serde(rename = "point_viewcontrol")]
    #[serde(borrow)]
    PointViewcontrol(PointViewcontrol<'a>),
    #[serde(rename = "prop_door_rotating")]
    #[serde(borrow)]
    PropDoorRotating(PropDoorRotating<'a>),
    #[serde(rename = "prop_dynamic")]
    #[serde(borrow)]
    PropDynamic(PropDynamic<'a>),
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
    #[serde(rename = "prop_ragdoll")]
    #[serde(borrow)]
    PropRagdoll(PropRagdoll<'a>),
    #[serde(rename = "shadow_control")]
    ShadowControl(ShadowControl),
    #[serde(rename = "sky_camera")]
    SkyCamera(SkyCamera),
    #[serde(rename = "trigger_gravity")]
    #[serde(borrow)]
    TriggerGravity(TriggerGravity<'a>),
    #[serde(rename = "trigger_hurt")]
    #[serde(borrow)]
    TriggerHurt(TriggerHurt<'a>),
    #[serde(rename = "trigger_look")]
    #[serde(borrow)]
    TriggerLook(TriggerLook<'a>),
    #[serde(rename = "trigger_multiple")]
    #[serde(borrow)]
    TriggerMultiple(TriggerMultiple<'a>),
    #[serde(rename = "trigger_once")]
    #[serde(borrow)]
    TriggerOnce(TriggerOnce<'a>),
    #[serde(rename = "trigger_proximity")]
    #[serde(borrow)]
    TriggerProximity(TriggerProximity<'a>),
    #[serde(rename = "trigger_push")]
    #[serde(borrow)]
    TriggerPush(TriggerPush<'a>),
    #[serde(rename = "trigger_soundscape")]
    #[serde(borrow)]
    TriggerSoundscape(TriggerSoundscape<'a>),
    #[serde(rename = "trigger_teleport")]
    #[serde(borrow)]
    TriggerTeleport(TriggerTeleport<'a>),
    #[serde(rename = "trigger_vphysics_motion")]
    #[serde(borrow)]
    TriggerVphysicsMotion(TriggerVphysicsMotion<'a>),
    #[serde(rename = "trigger_wind")]
    #[serde(borrow)]
    TriggerWind(TriggerWind<'a>),
    #[serde(rename = "water_lod_control")]
    #[serde(borrow)]
    WaterLodControl(WaterLodControl<'a>),
    #[serde(rename = "weapon_ak47")]
    #[serde(borrow)]
    WeaponAk47(WeaponAk47<'a>),
    #[serde(rename = "weapon_awp")]
    #[serde(borrow)]
    WeaponAwp(WeaponAwp<'a>),
    #[serde(rename = "weapon_deagle")]
    #[serde(borrow)]
    WeaponDeagle(WeaponDeagle<'a>),
    #[serde(rename = "weapon_elite")]
    #[serde(borrow)]
    WeaponElite(WeaponElite<'a>),
    #[serde(rename = "weapon_famas")]
    WeaponFamas(WeaponFamas),
    #[serde(rename = "weapon_fiveseven")]
    #[serde(borrow)]
    WeaponFiveseven(WeaponFiveseven<'a>),
    #[serde(rename = "weapon_flashbang")]
    #[serde(borrow)]
    WeaponFlashbang(WeaponFlashbang<'a>),
    #[serde(rename = "weapon_g3sg1")]
    WeaponG3sg1(WeaponG3sg1),
    #[serde(rename = "weapon_glock")]
    #[serde(borrow)]
    WeaponGlock(WeaponGlock<'a>),
    #[serde(rename = "weapon_hegrenade")]
    #[serde(borrow)]
    WeaponHegrenade(WeaponHegrenade<'a>),
    #[serde(rename = "weapon_knife")]
    #[serde(borrow)]
    WeaponKnife(WeaponKnife<'a>),
    #[serde(rename = "weapon_m249")]
    #[serde(borrow)]
    WeaponM249(WeaponM249<'a>),
    #[serde(rename = "weapon_m3")]
    #[serde(borrow)]
    WeaponM3(WeaponM3<'a>),
    #[serde(rename = "weapon_m4a1")]
    #[serde(borrow)]
    WeaponM4a1(WeaponM4a1<'a>),
    #[serde(rename = "weapon_mac10")]
    WeaponMac10(WeaponMac10),
    #[serde(rename = "weapon_p228")]
    #[serde(borrow)]
    WeaponP228(WeaponP228<'a>),
    #[serde(rename = "weapon_p90")]
    #[serde(borrow)]
    WeaponP90(WeaponP90<'a>),
    #[serde(rename = "weapon_scout")]
    #[serde(borrow)]
    WeaponScout(WeaponScout<'a>),
    #[serde(rename = "weapon_sg550")]
    WeaponSg550(WeaponSg550),
    #[serde(rename = "weapon_smokegrenade")]
    #[serde(borrow)]
    WeaponSmokegrenade(WeaponSmokegrenade<'a>),
    #[serde(rename = "weapon_tmp")]
    WeaponTmp(WeaponTmp),
    #[serde(rename = "weapon_ump45")]
    #[serde(borrow)]
    WeaponUmp45(WeaponUmp45<'a>),
    #[serde(rename = "weapon_usp")]
    #[serde(borrow)]
    WeaponUsp(WeaponUsp<'a>),
    #[serde(rename = "weapon_xm1014")]
    #[serde(borrow)]
    WeaponXm1014(WeaponXm1014<'a>),
    #[serde(rename = "worldspawn")]
    #[serde(borrow)]
    Worldspawn(Worldspawn<'a>),
}
#[derive(Debug, Clone, Deserialize)]
pub struct AmbientGeneric<'a> {
    #[serde(default)]
    pub cspinup: Option<i32>,
    #[serde(default)]
    pub fadein: Option<i32>,
    #[serde(default)]
    pub fadeinsecs: Option<i32>,
    #[serde(default)]
    pub fadeout: Option<i32>,
    #[serde(default)]
    pub fadeoutsecs: Option<i32>,
    pub health: u8,
    #[serde(default)]
    pub lfomodpitch: Option<i32>,
    #[serde(default)]
    pub lfomodvol: Option<i32>,
    #[serde(default)]
    pub lforate: Option<i32>,
    #[serde(default)]
    pub lfotype: Option<i32>,
    pub message: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub no_decomp: bool,
    pub origin: Vector,
    pub pitch: i32,
    pub pitchstart: i32,
    #[serde(default)]
    pub preset: Option<i32>,
    pub radius: f32,
    #[serde(default)]
    pub sourceentityname: Option<&'a str>,
    pub spawnflags: u32,
    #[serde(default)]
    pub spindown: Option<i32>,
    #[serde(default)]
    pub spinup: Option<i32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub volstart: Option<i32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Cycler<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    pub model: &'a str,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub sequence: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub skin: bool,
    pub spawnflags: u32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvBeam<'a> {
    pub boltwidth: f32,
    #[serde(default)]
    pub damage: Option<f32>,
    #[serde(default)]
    pub decalname: Option<&'a str>,
    #[serde(default)]
    pub framerate: Option<u8>,
    #[serde(default)]
    pub framestart: Option<i32>,
    #[serde(default)]
    pub hdrcolorscale: Option<f32>,
    pub life: f32,
    pub lightningend: &'a str,
    pub lightningstart: &'a str,
    #[serde(default)]
    pub noiseamplitude: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub radius: f32,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    pub spawnflags: u32,
    pub striketime: f32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub texture: &'a str,
    pub texturescroll: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub touchtype: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvBubbles<'a> {
    pub current: f32,
    pub density: i32,
    pub frequency: i32,
    pub model: &'a str,
    pub spawnflags: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvDetailController {
    pub angles: Angles,
    pub fademaxdist: u16,
    pub fademindist: u16,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvEmbers<'a> {
    pub angles: Angles,
    pub density: i32,
    pub lifetime: i32,
    pub model: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub particletype: bool,
    pub rendercolor: Color,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub speed: i32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvEntityMaker<'a> {
    pub angles: Angles,
    pub entitytemplate: &'a str,
    pub onentityspawned: &'a str,
    pub origin: Vector,
    pub postspawndirection: Vector,
    pub postspawndirectionvariance: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub postspawninheritangles: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub postspawnspeed: bool,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvExplosion<'a> {
    pub fireballsprite: &'a str,
    pub imagnitude: u16,
    #[serde(default)]
    pub iradiusoverride: Option<u16>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub rendermode: u8,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFade<'a> {
    pub duration: f32,
    pub holdtime: f32,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFire<'a> {
    pub damagescale: f32,
    pub fireattack: f32,
    #[serde(default)]
    pub firedecay: Option<u8>,
    pub firesize: f32,
    #[serde(default)]
    pub firetype: Option<i32>,
    pub health: u16,
    pub ignitionpoint: f32,
    pub origin: Vector,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFireTrail<'a> {
    pub origin: Vector,
    pub parentname: &'a str,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFiresource {
    pub firedamage: f32,
    pub fireradius: f32,
    pub origin: Vector,
    pub spawnflags: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvFogController<'a> {
    pub angles: Angles,
    pub farz: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub fogblend: bool,
    pub fogcolor: Color,
    pub fogcolor2: Color,
    pub fogdir: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub fogenable: bool,
    pub fogend: f32,
    #[serde(default)]
    pub foglerptime: Option<f32>,
    #[serde(default)]
    pub fogmaxdensity: Option<f32>,
    pub fogstart: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub use_angles: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvHudhint<'a> {
    pub message: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvLaser<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub current: bool,
    pub damage: f32,
    #[serde(default)]
    pub density: Option<u8>,
    pub dissolvetype: i32,
    #[serde(default)]
    pub frequency: Option<u8>,
    #[serde(default)]
    pub hdrcolorscale: Option<f32>,
    #[serde(default)]
    pub lasertarget: Option<&'a str>,
    #[serde(default)]
    pub noiseamplitude: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub texture: &'a str,
    pub texturescroll: u8,
    pub width: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvLightglow {
    pub angles: Angles,
    pub glowproxysize: f32,
    #[serde(default)]
    pub hdrcolorscale: Option<f32>,
    pub horizontalglowsize: u16,
    pub maxdist: u16,
    pub mindist: u16,
    pub origin: Vector,
    #[serde(default)]
    pub outermaxdist: Option<u16>,
    pub rendercolor: Color,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub verticalglowsize: u16,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvPhysexplosion<'a> {
    pub magnitude: f32,
    pub origin: Vector,
    pub radius: f32,
    pub spawnflags: u32,
    pub targetentityname: &'a str,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvProjectedtexture<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub cameraspace: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub enableshadows: bool,
    pub farz: f32,
    pub lightcolor: LightColor,
    pub lightfov: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub lightonlytarget: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub lightworld: bool,
    pub nearz: f32,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub shadowquality: i32,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvScreenoverlay<'a> {
    pub origin: Vector,
    pub overlayname1: &'a str,
    pub overlaytime1: f32,
    pub overlaytime10: f32,
    pub overlaytime2: f32,
    pub overlaytime3: f32,
    pub overlaytime4: f32,
    pub overlaytime5: f32,
    pub overlaytime6: f32,
    pub overlaytime7: f32,
    pub overlaytime8: f32,
    pub overlaytime9: f32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvShake<'a> {
    pub amplitude: f32,
    pub duration: f32,
    pub frequency: f32,
    pub origin: Vector,
    pub radius: f32,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvShooter<'a> {
    pub angles: Angles,
    pub delay: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    pub gibangles: Vector,
    #[serde(default)]
    pub gibanglevelocity: Option<f32>,
    pub gibgravityscale: f32,
    pub m_flgiblife: f32,
    pub m_flvariance: f32,
    pub m_flvelocity: u16,
    pub m_igibs: u32,
    #[serde(default)]
    pub massoverride: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nogibshadows: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    #[serde(default)]
    pub shootmodel: Option<&'a str>,
    pub shootsounds: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub simulation: bool,
    pub skin: i32,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSmokestack<'a> {
    pub angles: Angles,
    pub basespread: u8,
    pub endsize: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub initialstate: bool,
    pub jetlength: u8,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub rate: u8,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(default)]
    pub roll: Option<f32>,
    pub smokematerial: &'a str,
    pub speed: u8,
    pub spreadspeed: u8,
    pub startsize: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub twist: Option<u8>,
    #[serde(default)]
    pub windangle: Option<u8>,
    #[serde(default)]
    pub windspeed: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscape<'a> {
    pub origin: Vector,
    #[serde(default)]
    pub position0: Option<&'a str>,
    #[serde(default)]
    pub position1: Option<&'a str>,
    #[serde(default)]
    pub position2: Option<&'a str>,
    #[serde(default)]
    pub position3: Option<&'a str>,
    #[serde(default)]
    pub position4: Option<&'a str>,
    #[serde(default)]
    pub position5: Option<&'a str>,
    #[serde(default)]
    pub position6: Option<&'a str>,
    #[serde(default)]
    pub position7: Option<&'a str>,
    pub radius: f32,
    pub soundscape: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscapeProxy<'a> {
    pub mainsoundscapename: &'a str,
    pub origin: Vector,
    pub radius: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSoundscapeTriggerable<'a> {
    pub origin: Vector,
    #[serde(default)]
    pub position0: Option<&'a str>,
    #[serde(default)]
    pub position1: Option<&'a str>,
    #[serde(default)]
    pub position2: Option<&'a str>,
    #[serde(default)]
    pub position3: Option<&'a str>,
    #[serde(default)]
    pub position4: Option<&'a str>,
    #[serde(default)]
    pub position5: Option<&'a str>,
    #[serde(default)]
    pub position6: Option<&'a str>,
    #[serde(default)]
    pub position7: Option<&'a str>,
    pub radius: f32,
    pub soundscape: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpark<'a> {
    pub angles: Angles,
    pub magnitude: u8,
    #[serde(default)]
    pub maxdelay: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub traillength: u8,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSprite<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    pub framerate: f32,
    pub glowproxysize: f32,
    #[serde(default)]
    pub hdrcolorscale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(default)]
    pub mindxlevel: Option<u8>,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(default)]
    pub renderfx: Option<u8>,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(default)]
    pub scale: Option<f32>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSpritetrail<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    pub endwidth: f32,
    pub lifetime: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub rendermode: u8,
    pub spritename: &'a str,
    pub startwidth: f32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSteam<'a> {
    pub angles: Angles,
    pub endsize: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub initialstate: bool,
    pub jetlength: u16,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub r#type: bool,
    pub rate: u8,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(default)]
    pub rollspeed: Option<u8>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub speed: u8,
    pub spreadspeed: u8,
    pub startsize: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvSun<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub hdrcolorscale: Option<f32>,
    #[serde(default)]
    pub horzsize0: Option<u8>,
    #[serde(default)]
    pub horzsize1: Option<u8>,
    #[serde(default)]
    pub horzsize2: Option<u8>,
    #[serde(default)]
    pub horzsize3: Option<u8>,
    #[serde(default)]
    pub material: Option<&'a str>,
    #[serde(default)]
    pub material0: Option<&'a str>,
    #[serde(default)]
    pub material1: Option<&'a str>,
    #[serde(default)]
    pub material2: Option<&'a str>,
    #[serde(default)]
    pub material3: Option<&'a str>,
    #[serde(default)]
    pub numlayers: Option<u8>,
    pub origin: Vector,
    #[serde(default)]
    pub overlaycolor: Option<Color>,
    #[serde(default)]
    pub overlaymaterial: Option<&'a str>,
    #[serde(default)]
    pub overlaysize: Option<i32>,
    #[serde(default)]
    pub pitch: Option<f32>,
    pub rendercolor: Color,
    pub size: i32,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub use_angles: Option<i32>,
    #[serde(default)]
    pub vertsize0: Option<u8>,
    #[serde(default)]
    pub vertsize1: Option<u8>,
    #[serde(default)]
    pub vertsize2: Option<u8>,
    #[serde(default)]
    pub vertsize3: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvTonemapController<'a> {
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct EnvWind {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub gustdirchange: i32,
    #[serde(default)]
    pub gustduration: Option<f32>,
    pub maxgust: i32,
    pub maxgustdelay: f32,
    pub maxwind: i32,
    pub mingust: i32,
    pub mingustdelay: f32,
    pub minwind: i32,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorClass<'a> {
    pub filterclass: &'a str,
    pub negated: Negated,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterActivatorName<'a> {
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub negated: Negated,
    #[serde(default)]
    pub onfail: Option<&'a str>,
    #[serde(default)]
    pub onpass: Option<&'a str>,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterDamageType<'a> {
    pub damagetype: i32,
    pub negated: Negated,
    pub origin: Vector,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FilterMulti<'a> {
    pub filter01: &'a str,
    #[serde(default)]
    pub filter02: Option<&'a str>,
    #[serde(default)]
    pub filter03: Option<&'a str>,
    #[serde(default)]
    pub filter04: Option<&'a str>,
    #[serde(default)]
    pub filter05: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub filtertype: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub negated: bool,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncAreaportal<'a> {
    pub portalnumber: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub portalversion: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startopen: bool,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncAreaportalwindow<'a> {
    pub fadedist: u16,
    pub fadestartdist: u16,
    pub portalnumber: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub portalversion: bool,
    #[serde(default)]
    pub target: Option<&'a str>,
    pub translucencylimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBombTarget<'a> {
    pub bombexplode: &'a str,
    pub model: &'a str,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBreakable<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub delay: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(default)]
    pub explodemagnitude: Option<i32>,
    #[serde(default)]
    pub exploderadius: Option<u16>,
    #[serde(default)]
    pub explosion: Option<i32>,
    pub gibdir: Vector,
    pub health: u32,
    #[serde(default)]
    pub material: Option<i32>,
    #[serde(default)]
    pub minhealthdmg: Option<i32>,
    pub model: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nodamageforces: bool,
    #[serde(default)]
    pub onbreak: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub performancemode: bool,
    pub physdamagescale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub pressuredelay: bool,
    #[serde(default)]
    pub propdata: Option<i32>,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub spawnobject: Option<i32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBreakableSurf<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub error: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub explodedamage: bool,
    pub explodemagnitude: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub exploderadius: bool,
    pub explosion: i32,
    pub fragility: i32,
    pub gibdir: Vector,
    pub health: u8,
    pub lowerleft: Vector,
    pub lowerright: Vector,
    pub material: i32,
    pub model: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nodamageforces: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub performancemode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub pressuredelay: bool,
    #[serde(default)]
    pub propdata: Option<i32>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub spawnobject: i32,
    pub surfacetype: i32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub upperleft: Vector,
    pub upperright: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBrush<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(default)]
    pub inputfilter: Option<u8>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub invert_exclusion: bool,
    pub model: &'a str,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(default)]
    pub renderfx: Option<u8>,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub solidbsp: bool,
    pub solidity: u8,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub speed: Option<u8>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
    #[serde(default)]
    pub wait: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncButton<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub health: bool,
    pub lip: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub locked_sentence: bool,
    #[serde(default)]
    pub locked_sound: Option<u8>,
    pub model: &'a str,
    pub movedir: Vector,
    #[serde(default)]
    pub ondamaged: Option<&'a str>,
    #[serde(default)]
    pub onin: Option<&'a str>,
    #[serde(default)]
    pub onout: Option<&'a str>,
    pub onpressed: &'a str,
    #[serde(default)]
    pub onuselocked: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub renderfx: u8,
    pub rendermode: u8,
    pub sounds: i32,
    pub spawnflags: u32,
    pub speed: f32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub unlocked_sentence: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub unlocked_sound: bool,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncBuyzone<'a> {
    pub model: &'a str,
    #[serde(default)]
    pub team: Option<u8>,
    #[serde(default)]
    pub teamnum: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncClipVphysics<'a> {
    pub model: &'a str,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncConveyor<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub model: &'a str,
    pub movedir: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    pub spawnflags: u32,
    pub speed: u16,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDoor<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(default)]
    pub dmg: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub forceclosed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub health: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub ignoredebris: bool,
    #[serde(default)]
    pub lip: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub locked_sentence: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub locked_sound: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub loopmovesound: bool,
    pub model: &'a str,
    pub movedir: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub no_decomp: bool,
    #[serde(default)]
    pub noise1: Option<&'a str>,
    #[serde(default)]
    pub noise2: Option<&'a str>,
    #[serde(default)]
    pub onclose: Option<&'a str>,
    #[serde(default)]
    pub onfullyclosed: Option<&'a str>,
    #[serde(default)]
    pub onfullyopen: Option<&'a str>,
    #[serde(default)]
    pub onopen: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(default)]
    pub renderfx: Option<u8>,
    pub rendermode: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub sounds: bool,
    pub spawnflags: u32,
    #[serde(default)]
    pub spawnpos: Option<i32>,
    pub speed: u16,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub unlocked_sentence: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub unlocked_sound: bool,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDoorRotating<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub _minlight: bool,
    pub angles: Angles,
    #[serde(default)]
    pub chainstodoor: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub distance: u8,
    pub dmg: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub forceclosed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub health: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub ignoredebris: bool,
    pub lip: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    pub locked_sentence: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub loopmovesound: bool,
    pub model: &'a str,
    #[serde(default)]
    pub noise1: Option<&'a str>,
    #[serde(default)]
    pub noise2: Option<&'a str>,
    #[serde(default)]
    pub onfullyclosed: Option<&'a str>,
    #[serde(default)]
    pub onfullyopen: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub solidbsp: bool,
    pub spawnflags: u32,
    #[serde(default)]
    pub spawnpos: Option<i32>,
    pub speed: u16,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub unlocked_sentence: bool,
    pub wait: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDustcloud<'a> {
    pub alpha: u8,
    pub color: Color,
    pub distmax: u16,
    #[serde(deserialize_with = "deserialize_bool")]
    pub frozen: bool,
    pub lifetimemax: u8,
    pub lifetimemin: u8,
    pub model: &'a str,
    pub sizemax: u8,
    pub sizemin: u8,
    pub spawnrate: u16,
    pub speedmax: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncDustmotes<'a> {
    pub alpha: u8,
    pub color: Color,
    pub distmax: u16,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub fallspeed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub frozen: bool,
    pub lifetimemax: u8,
    pub lifetimemin: u8,
    pub model: &'a str,
    pub sizemax: u8,
    pub sizemin: u8,
    pub spawnrate: u32,
    pub speedmax: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncFishPool<'a> {
    pub fish_count: i32,
    pub max_range: u16,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub skin: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncFootstepControl<'a> {
    pub destination: &'a str,
    pub model: &'a str,
    pub source: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncHostageRescue<'a> {
    pub model: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncIllusionary<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    pub model: &'a str,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(default)]
    pub renderfx: Option<u8>,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(default)]
    pub skin: Option<i32>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub speed: Option<u8>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub wait: Option<u8>,
    #[serde(default)]
    pub zhlt_lightflags: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncLod<'a> {
    pub disappeardist: u16,
    pub model: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub solid: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncMonitor<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub inputfilter: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub invert_exclusion: bool,
    pub model: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub solidbsp: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub solidity: bool,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    pub target: &'a str,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub vrad_brush_cast_shadows: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncMovelinear<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub blockdamage: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    pub model: &'a str,
    pub movedir: Vector,
    pub movedistance: u16,
    #[serde(default)]
    pub onfullyclosed: Option<&'a str>,
    #[serde(default)]
    pub onfullyopen: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    pub rendermode: u8,
    pub spawnflags: u32,
    pub speed: u8,
    pub startposition: f32,
    #[serde(default)]
    pub startsound: Option<&'a str>,
    #[serde(default)]
    pub stopsound: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncOccluder<'a> {
    pub model: &'a str,
    pub occludernumber: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startactive: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysbox<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub damagetoenablemotion: Option<i32>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub damagetype: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(default)]
    pub explodemagnitude: Option<i32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub exploderadius: bool,
    #[serde(default)]
    pub explosion: Option<i32>,
    #[serde(default)]
    pub forcetoenablemotion: Option<f32>,
    pub gibdir: Vector,
    #[serde(default)]
    pub health: Option<u16>,
    #[serde(default)]
    pub massscale: Option<f32>,
    #[serde(default)]
    pub material: Option<i32>,
    pub model: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nodamageforces: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub notsolid: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub performancemode: bool,
    pub preferredcarryangles: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub pressuredelay: bool,
    #[serde(default)]
    pub propdata: Option<i32>,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    pub spawnflags: u32,
    #[serde(default)]
    pub spawnobject: Option<i32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPhysboxMultiplayer<'a> {
    #[serde(default)]
    pub _minlight: Option<u8>,
    pub damagetoenablemotion: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub damagetype: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub explodedamage: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub explodemagnitude: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub exploderadius: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub explosion: bool,
    pub forcetoenablemotion: f32,
    pub gibdir: Angles,
    pub health: u8,
    pub massscale: f32,
    pub material: u8,
    pub model: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub notsolid: bool,
    #[serde(default)]
    pub onawakened: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub performancemode: bool,
    pub preferredcarryangles: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub propdata: bool,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub renderfx: u8,
    pub rendermode: u8,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub spawnobject: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncPrecipitation<'a> {
    pub model: &'a str,
    #[serde(default)]
    pub preciptype: Option<i32>,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(default)]
    pub renderfx: Option<u8>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotButton<'a> {
    pub angles: Angles,
    pub distance: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub health: bool,
    pub model: &'a str,
    pub onpressed: &'a str,
    pub origin: Vector,
    pub sounds: i32,
    pub spawnflags: u32,
    pub speed: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub wait: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncRotating<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    pub dmg: f32,
    pub fanfriction: f32,
    pub maxspeed: f32,
    #[serde(default)]
    pub message: Option<&'a str>,
    pub model: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub no_decomp: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    pub rendermode: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub solidbsp: bool,
    pub spawnflags: u32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub volume: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncSmokevolume<'a> {
    pub color1: Color,
    pub color2: Color,
    pub density: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub densityrampspeed: bool,
    pub material: &'a str,
    pub model: &'a str,
    pub movementspeed: u8,
    pub particledrawwidth: u8,
    pub particlespacingdistance: u8,
    pub rotationspeed: u8,
    pub spawnflags: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTracktrain<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub _minlight: bool,
    pub bank: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub dmg: f32,
    pub height: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub manualaccelspeed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub manualdecelspeed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub manualspeedchanges: bool,
    pub model: &'a str,
    #[serde(default)]
    pub movesound: Option<&'a str>,
    pub movesoundmaxpitch: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    pub movesoundmaxtime: bool,
    pub movesoundminpitch: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    pub movesoundmintime: bool,
    pub orientationtype: i32,
    pub origin: Vector,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    pub spawnflags: u32,
    pub speed: u16,
    #[serde(default)]
    pub startsound: Option<&'a str>,
    pub startspeed: f32,
    #[serde(default)]
    pub stopsound: Option<&'a str>,
    pub target: &'a str,
    pub targetname: &'a str,
    pub velocitytype: i32,
    pub volume: i32,
    pub wheels: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncTrain<'a> {
    pub _minlight: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub dmg: f32,
    pub model: &'a str,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    pub spawnflags: u32,
    pub speed: u16,
    pub target: &'a str,
    pub targetname: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub texframeindex: bool,
    pub volume: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWall<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    pub model: &'a str,
    #[serde(default)]
    pub origin: Option<Vector>,
    pub renderamt: i32,
    pub rendercolor: Color,
    #[serde(default)]
    pub renderfx: Option<u8>,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub zhlt_lightflags: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWallToggle<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub model: &'a str,
    #[serde(default)]
    pub origin: Option<Vector>,
    pub renderamt: u8,
    pub rendercolor: Color,
    pub renderfx: u8,
    pub rendermode: u8,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct FuncWaterAnalog<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    pub model: &'a str,
    #[serde(default)]
    pub movedir: Option<Vector>,
    #[serde(default)]
    pub movedistance: Option<u8>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(default)]
    pub speed: Option<u8>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startposition: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub waveheight: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GamePlayerEquip<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub ammo_338mag: bool,
    #[serde(default)]
    pub ammo_45acp: Option<u8>,
    #[serde(default)]
    pub ammo_50ae: Option<u8>,
    #[serde(default)]
    pub ammo_762mm: Option<u8>,
    #[serde(default)]
    pub ammo_9mm: Option<u8>,
    #[serde(default)]
    pub ammo_buckshot: Option<u8>,
    #[serde(default)]
    pub item_assaultsuit: Option<u8>,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub weapon_awp: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub weapon_deagle: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub weapon_glock: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub weapon_hegrenade: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub weapon_knife: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub weapon_m3: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub weapon_p90: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub weapon_scout: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub weapon_usp: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameText<'a> {
    pub channel: i32,
    pub color: Color,
    pub color2: Color,
    pub effect: i32,
    pub fadein: f32,
    pub fadeout: f32,
    pub fxtime: f32,
    pub holdtime: f32,
    pub message: &'a str,
    pub origin: Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameUi<'a> {
    pub fieldofview: f32,
    pub origin: Vector,
    pub playeroff: &'a str,
    pub pressedback: &'a str,
    pub pressedforward: &'a str,
    pub pressedmoveleft: &'a str,
    pub pressedmoveright: &'a str,
    pub spawnflags: u32,
    pub targetname: &'a str,
    pub unpressedback: &'a str,
    pub unpressedforward: &'a str,
    pub unpressedmoveleft: &'a str,
    pub unpressedmoveright: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct GameWeaponManager<'a> {
    pub ammomod: f32,
    pub maxpieces: i32,
    pub origin: Vector,
    pub weaponname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct HostageEntity<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub hostagetype: bool,
    #[serde(default)]
    pub model: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub spawnflags: Option<u32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoCameraLink<'a> {
    pub origin: Vector,
    #[serde(default)]
    pub pointcamera: Option<&'a str>,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoLadder {
    #[serde(rename = "maxs.x")]
    pub maxs_x: f32,
    #[serde(rename = "maxs.y")]
    pub maxs_y: f32,
    #[serde(rename = "maxs.z")]
    pub maxs_z: f32,
    #[serde(rename = "mins.x")]
    pub mins_x: f32,
    #[serde(rename = "mins.y")]
    pub mins_y: f32,
    #[serde(rename = "mins.z")]
    pub mins_z: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoLightingRelative<'a> {
    pub lightinglandmark: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoMapParameters {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub bombradius: u16,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub buying: bool,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNode {
    pub nodeid: u16,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoNodeHint {
    pub angles: Angles,
    pub hinttype: u16,
    pub ignorefacing: u8,
    pub maximumstate: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    pub minimumstate: bool,
    pub nodefov: u8,
    pub nodeid: u8,
    pub origin: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub starthintdisabled: bool,
    pub targetnode: i32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoParticleSystem<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    pub cpoint1_parent: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub cpoint2_parent: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub cpoint3_parent: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub cpoint4_parent: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub cpoint5_parent: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub cpoint6_parent: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub cpoint7_parent: bool,
    pub effect_name: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub flag_as_weather: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub start_active: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerCounterterrorist {
    pub angles: Angles,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerLogo {
    pub angles: Angles,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerStart {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoPlayerTerrorist {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub no_decomp: bool,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTarget<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct InfoTeleportDestination<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Infodecal<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub lowpriority: bool,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub texture: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct KeyframeRope<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub barbed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub breakable: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub collide: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub dangling: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub movespeed: u8,
    #[serde(default)]
    pub nextkey: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nowind: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub r#type: bool,
    pub ropematerial: &'a str,
    pub slack: f32,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub subdiv: u8,
    pub targetname: &'a str,
    pub texturescale: u8,
    pub width: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Light<'a> {
    #[serde(default)]
    pub _constant_attn: Option<f32>,
    #[serde(default)]
    pub _distance: Option<u16>,
    #[serde(default)]
    pub _fifty_percent_distance: Option<f32>,
    #[serde(default)]
    pub _hardfalloff: Option<u16>,
    pub _light: LightColor,
    #[serde(default)]
    pub _lighthdr: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub _lightscalehdr: bool,
    #[serde(default)]
    pub _linear_attn: Option<f32>,
    pub _quadratic_attn: f32,
    #[serde(default)]
    pub _zero_percent_distance: Option<f32>,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub defaultstyle: Option<i32>,
    #[serde(default)]
    pub ontimer: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub style: Option<i32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LightEnvironment<'a> {
    pub _ambient: LightColor,
    #[serde(default)]
    pub _ambienthdr: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub _ambientscalehdr: bool,
    #[serde(default)]
    pub _diffuse_light: Option<LightColor>,
    #[serde(default)]
    pub _diffuse_lighting: Option<LightColor>,
    pub _light: LightColor,
    #[serde(default)]
    pub _lighthdr: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub _lightscalehdr: bool,
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub pitch: Option<Angles>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub style: Option<i32>,
    #[serde(default)]
    pub sunspreadangle: Option<u8>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LightSpot<'a> {
    pub _cone: u8,
    #[serde(default)]
    pub _cone2: Option<u8>,
    #[serde(default)]
    pub _constant_attn: Option<f32>,
    #[serde(default)]
    pub _distance: Option<u32>,
    pub _exponent: f32,
    #[serde(default)]
    pub _fifty_percent_distance: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub _hardfalloff: bool,
    pub _inner_cone: u8,
    pub _light: LightColor,
    #[serde(default)]
    pub _lighthdr: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub _lightscalehdr: bool,
    #[serde(default)]
    pub _linear_attn: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub _quadratic_attn: bool,
    #[serde(default)]
    pub _zero_percent_distance: Option<f32>,
    pub angles: Angles,
    #[serde(default)]
    pub defaultstyle: Option<i32>,
    pub origin: Vector,
    pub pitch: Angles,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub style: Option<i32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicAuto<'a> {
    #[serde(default)]
    pub onloadgame: Option<&'a str>,
    pub onmapspawn: &'a str,
    #[serde(default)]
    pub onnewgame: Option<&'a str>,
    pub origin: Vector,
    pub spawnflags: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicBranch<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub initialvalue: bool,
    #[serde(default)]
    pub onfalse: Option<&'a str>,
    pub ontrue: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicCase<'a> {
    #[serde(default)]
    pub case01: Option<u8>,
    #[serde(default)]
    pub case02: Option<u8>,
    #[serde(default)]
    pub case03: Option<u8>,
    #[serde(default)]
    pub case04: Option<u8>,
    #[serde(default)]
    pub case05: Option<u8>,
    #[serde(default)]
    pub case06: Option<u8>,
    #[serde(default)]
    pub case07: Option<u8>,
    #[serde(default)]
    pub case08: Option<u8>,
    #[serde(default)]
    pub case09: Option<u8>,
    #[serde(default)]
    pub case10: Option<u8>,
    #[serde(default)]
    pub case11: Option<u8>,
    #[serde(default)]
    pub case12: Option<u8>,
    #[serde(default)]
    pub case13: Option<u8>,
    #[serde(default)]
    pub case14: Option<u8>,
    #[serde(default)]
    pub case15: Option<u8>,
    #[serde(default)]
    pub case16: Option<u8>,
    pub oncase01: &'a str,
    pub oncase02: &'a str,
    pub oncase03: &'a str,
    pub oncase04: &'a str,
    #[serde(default)]
    pub oncase05: Option<&'a str>,
    #[serde(default)]
    pub oncase06: Option<&'a str>,
    #[serde(default)]
    pub oncase07: Option<&'a str>,
    #[serde(default)]
    pub oncase08: Option<&'a str>,
    #[serde(default)]
    pub oncase09: Option<&'a str>,
    #[serde(default)]
    pub oncase10: Option<&'a str>,
    #[serde(default)]
    pub oncase11: Option<&'a str>,
    #[serde(default)]
    pub oncase12: Option<&'a str>,
    #[serde(default)]
    pub oncase13: Option<&'a str>,
    #[serde(default)]
    pub oncase14: Option<&'a str>,
    #[serde(default)]
    pub oncase15: Option<&'a str>,
    #[serde(default)]
    pub oncase16: Option<&'a str>,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicCompare<'a> {
    #[serde(deserialize_with = "deserialize_bool")]
    pub comparevalue: bool,
    pub ongreaterthan: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicMeasureMovement<'a> {
    pub measurereference: &'a str,
    pub measuretarget: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub measuretype: bool,
    pub origin: Vector,
    pub target: &'a str,
    pub targetname: &'a str,
    pub targetreference: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    pub targetscale: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicRelay<'a> {
    pub ontrigger: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct LogicTimer<'a> {
    #[serde(default)]
    pub lowerrandombound: Option<u8>,
    pub ontimer: &'a str,
    #[serde(default)]
    pub ontimerhigh: Option<&'a str>,
    #[serde(default)]
    pub ontimerlow: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub refiretime: Option<f32>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub upperrandombound: Option<u8>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub userandomtime: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MathCounter<'a> {
    pub max: f32,
    #[serde(default)]
    pub min: Option<f32>,
    pub onhitmax: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub outvalue: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startvalue: bool,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct MoveRope<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub barbed: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub breakable: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub collide: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub dangling: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub movespeed: u8,
    #[serde(default)]
    pub nextkey: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nowind: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub positioninterpolator: u8,
    #[serde(default)]
    pub r#type: Option<u8>,
    pub ropematerial: &'a str,
    pub slack: u8,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub subdiv: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub texturescale: u8,
    pub width: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PathTrack<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub onpass: Option<&'a str>,
    #[serde(default)]
    pub onuser1: Option<&'a str>,
    pub orientationtype: i32,
    pub origin: Vector,
    #[serde(default)]
    pub radius: Option<f32>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub speed: Option<u16>,
    #[serde(default)]
    pub target: Option<&'a str>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysBallsocket<'a> {
    pub attach1: &'a str,
    #[serde(default)]
    pub forcelimit: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub torquelimit: Option<f32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysConstraint<'a> {
    pub angles: Angles,
    pub constraintsystem: &'a str,
    pub forcelimit: f32,
    pub origin: Vector,
    pub spawnflags: u32,
    pub torquelimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysConstraintsystem<'a> {
    pub additionaliterations: i32,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysHinge<'a> {
    pub attach1: &'a str,
    #[serde(default)]
    pub breaksound: Option<&'a str>,
    #[serde(default)]
    pub constraintsystem: Option<&'a str>,
    pub forcelimit: f32,
    pub hingeaxis: Vector,
    pub hingefriction: f32,
    pub origin: Vector,
    pub spawnflags: u32,
    pub systemloadscale: f32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub torquelimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysKeepupright<'a> {
    pub angles: Angles,
    pub angularlimit: f32,
    pub attach1: &'a str,
    pub origin: Vector,
    pub spawnflags: u32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysLengthconstraint<'a> {
    pub addlength: f32,
    pub angles: Angles,
    pub attach1: &'a str,
    pub attachpoint: Vector,
    pub constraintsystem: &'a str,
    pub forcelimit: f32,
    pub minlength: f32,
    pub origin: Vector,
    pub spawnflags: u32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub torquelimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysPulleyconstraint<'a> {
    pub addlength: f32,
    pub angles: Angles,
    #[serde(default)]
    pub attach1: Option<&'a str>,
    #[serde(default)]
    pub attach2: Option<&'a str>,
    #[serde(default)]
    pub constraintsystem: Option<&'a str>,
    pub forcelimit: f32,
    pub gearratio: f32,
    pub origin: Vector,
    pub position2: Vector,
    pub spawnflags: u32,
    pub torquelimit: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysRagdollconstraint<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub attach1: &'a str,
    pub forcelimit: f32,
    pub origin: Vector,
    pub spawnflags: u32,
    #[serde(default)]
    pub teleportfollowdistance: Option<f32>,
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
    pub angles: Angles,
    pub axis: Vector,
    pub force: f32,
    pub origin: Vector,
    pub radius: f32,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysThruster<'a> {
    pub angles: Angles,
    pub attach1: &'a str,
    pub force: f32,
    pub forcetime: f32,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PhysTorque<'a> {
    pub attach1: &'a str,
    pub axis: Vector,
    pub force: f32,
    pub forcetime: f32,
    pub origin: Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerSpeedmod<'a> {
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PlayerWeaponstrip<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointCamera<'a> {
    pub angles: Angles,
    pub fogcolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub fogenable: bool,
    pub fogend: u16,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub fogmaxdensity: bool,
    pub fogstart: u16,
    pub fov: u8,
    pub origin: Vector,
    #[serde(default)]
    pub resolution: Option<f32>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub targetname: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub usescreenaspectratio: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointClientcommand<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointDevshotCamera<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub cameraname: Option<&'a str>,
    pub fov: u8,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointServercommand<'a> {
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointSpotlight<'a> {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(default)]
    pub hdrcolorscale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(default)]
    pub mindxlevel: Option<i32>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(default)]
    pub renderamt: Option<u8>,
    pub rendercolor: Color,
    #[serde(default)]
    pub renderfx: Option<u8>,
    #[serde(default)]
    pub rendermode: Option<u8>,
    pub spawnflags: u32,
    pub spotlightlength: f32,
    pub spotlightwidth: f32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointSurroundtest<'a> {
    pub on2speakers: &'a str,
    pub on4speakers: &'a str,
    pub on51speakers: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointTemplate<'a> {
    #[serde(default)]
    pub boltwidth: Option<u8>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub damage: bool,
    #[serde(default)]
    pub decalname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub framerate: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub framestart: bool,
    #[serde(default)]
    pub hdrcolorscale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub life: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub noiseamplitude: bool,
    #[serde(default)]
    pub onentityspawned: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub radius: Option<u16>,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub striketime: bool,
    pub targetname: &'a str,
    pub template01: &'a str,
    #[serde(default)]
    pub template02: Option<&'a str>,
    #[serde(default)]
    pub template03: Option<&'a str>,
    #[serde(default)]
    pub template04: Option<&'a str>,
    #[serde(default)]
    pub template05: Option<&'a str>,
    #[serde(default)]
    pub texture: Option<&'a str>,
    #[serde(default)]
    pub texturescroll: Option<u8>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub touchtype: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointTesla<'a> {
    pub beamcount_max: i32,
    pub beamcount_min: i32,
    pub interval_max: f32,
    pub interval_min: f32,
    pub lifetime_max: f32,
    pub lifetime_min: f32,
    pub m_color: Color,
    pub m_flradius: u8,
    pub m_soundname: &'a str,
    pub origin: Vector,
    pub targetname: &'a str,
    pub texture: &'a str,
    pub thick_max: f32,
    pub thick_min: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PointViewcontrol<'a> {
    pub acceleration: u16,
    pub angles: Angles,
    pub deceleration: u16,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub interpolatepositiontoplayer: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub speed: bool,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDoorRotating<'a> {
    pub ajarangles: Vector,
    pub angles: Angles,
    pub axis: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    pub distance: f32,
    #[serde(default)]
    pub dmg: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub forceclosed: bool,
    pub hardware: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub health: bool,
    pub model: &'a str,
    #[serde(default)]
    pub onclose: Option<&'a str>,
    #[serde(default)]
    pub onopen: Option<&'a str>,
    #[serde(default)]
    pub opendir: Option<i32>,
    pub origin: Vector,
    pub returndelay: f32,
    pub skin: u8,
    #[serde(default)]
    pub soundcloseoverride: Option<&'a str>,
    #[serde(default)]
    pub soundmoveoverride: Option<&'a str>,
    #[serde(default)]
    pub soundopenoverride: Option<&'a str>,
    pub spawnflags: u32,
    #[serde(default)]
    pub spawnpos: Option<i32>,
    pub speed: u8,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamic<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub body: bool,
    #[serde(default)]
    pub defaultanim: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablebonefollowers: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub exploderadius: bool,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    pub fademindist: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub fadescale: bool,
    #[serde(default)]
    pub globalname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub health: bool,
    #[serde(default)]
    pub lightingoriginhack: Option<&'a str>,
    pub maxanimtime: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    pub minanimtime: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub model: &'a str,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(default)]
    pub ontakedamage: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub performancemode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub randomanimation: bool,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub renderfx: Option<u8>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(default)]
    pub setbodygroup: Option<u8>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub skin: bool,
    pub solid: u8,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropDynamicOverride<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub angles: Angles,
    #[serde(default)]
    pub defaultanim: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablebonefollowers: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub exploderadius: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    pub fademindist: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub fadescale: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub health: bool,
    pub maxanimtime: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    pub minanimtime: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub model: &'a str,
    #[serde(default)]
    pub modelscale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub performancemode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub pressuredelay: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub randomanimation: bool,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub setbodygroup: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub skin: bool,
    pub solid: u8,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysics<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub angles: Angles,
    #[serde(default)]
    pub damagetoenablemotion: Option<i32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub damagetype: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub exploderadius: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    pub fademindist: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub fadescale: bool,
    #[serde(default)]
    pub forcetoenablemotion: Option<f32>,
    pub inertiascale: f32,
    #[serde(default)]
    pub massscale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(default)]
    pub minhealthdmg: Option<i32>,
    pub model: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub performancemode: bool,
    pub physdamagescale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub pressuredelay: bool,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(default)]
    pub shadowcastdist: Option<u8>,
    #[serde(default)]
    pub skin: Option<u8>,
    pub spawnflags: u32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsMultiplayer<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub body: bool,
    #[serde(default)]
    pub damagetoenablemotion: Option<i32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub damagetype: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub explodedamage: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub exploderadius: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    pub fademindist: i32,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub forcetoenablemotion: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub health: bool,
    pub inertiascale: f32,
    #[serde(default)]
    pub massscale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(default)]
    pub minhealthdmg: Option<u8>,
    pub model: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub multiplayer_physics_mode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nodamageforces: bool,
    #[serde(default)]
    pub onawakened: Option<&'a str>,
    #[serde(default)]
    pub onbreak: Option<&'a str>,
    #[serde(default)]
    pub onhealthchanged: Option<&'a str>,
    #[serde(default)]
    pub onmotionenabled: Option<&'a str>,
    pub origin: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub performancemode: bool,
    pub physdamagescale: f32,
    #[serde(default)]
    pub physicsmode: Option<i32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub pressuredelay: bool,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(default)]
    pub skin: Option<u8>,
    #[serde(default)]
    pub solid: Option<u8>,
    pub spawnflags: u32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropPhysicsOverride<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub body: bool,
    pub damagetoenablemotion: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub damagetype: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub disableshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub explodedamage: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub exploderadius: bool,
    #[serde(default)]
    pub fademaxdist: Option<u16>,
    pub fademindist: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub fadescale: bool,
    pub forcetoenablemotion: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub health: bool,
    pub inertiascale: f32,
    pub massscale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub minhealthdmg: i32,
    pub model: &'a str,
    #[serde(default)]
    pub modelscale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub performancemode: bool,
    pub physdamagescale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub pressuredelay: bool,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub shadowcastdist: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub skin: bool,
    pub spawnflags: u32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct PropRagdoll<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    pub model: &'a str,
    #[serde(default)]
    pub modelscale: Option<f32>,
    pub origin: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderamt: bool,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub sequence: Option<i32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub skin: bool,
    #[serde(default)]
    pub solid: Option<u8>,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct ShadowControl {
    pub angles: Angles,
    pub color: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableallshadows: bool,
    pub distance: f32,
    pub origin: Vector,
}
#[derive(Debug, Clone, Deserialize)]
pub struct SkyCamera {
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub fogblend: bool,
    pub fogcolor: Color,
    pub fogcolor2: Color,
    pub fogdir: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub fogenable: bool,
    pub fogend: f32,
    #[serde(default)]
    pub fogmaxdensity: Option<f32>,
    pub fogstart: f32,
    pub origin: Vector,
    pub scale: i32,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub use_angles: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerGravity<'a> {
    pub gravity: f32,
    pub model: &'a str,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    pub origin: Vector,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerHurt<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub damage: f32,
    pub damagecap: f32,
    pub damagemodel: i32,
    pub damagetype: i32,
    pub model: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nodmgforce: bool,
    #[serde(default)]
    pub onhurtplayer: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerLook<'a> {
    pub fieldofview: f32,
    pub looktime: u8,
    pub model: &'a str,
    pub ontrigger: &'a str,
    pub origin: Vector,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
    pub target: &'a str,
    pub timeout: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerMultiple<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    #[serde(default)]
    pub onendtouchall: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouchall: Option<&'a str>,
    pub ontrigger: &'a str,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u32,
    #[serde(default)]
    pub speed: Option<u16>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdisabled: bool,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    pub wait: f32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerOnce<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub model: &'a str,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    pub ontrigger: &'a str,
    pub origin: Vector,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
    #[serde(default)]
    pub wait: Option<i32>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerProximity<'a> {
    pub model: &'a str,
    pub onstarttouch: &'a str,
    pub origin: Vector,
    pub radius: f32,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerPush<'a> {
    #[serde(default)]
    pub alternateticksfix: Option<f32>,
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    pub model: &'a str,
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub pushdir: Vector,
    pub spawnflags: u32,
    pub speed: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerSoundscape<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    pub model: &'a str,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    pub origin: Vector,
    pub soundscape: &'a str,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerTeleport<'a> {
    #[serde(default)]
    pub angles: Option<Angles>,
    #[serde(default)]
    pub filtername: Option<&'a str>,
    #[serde(default)]
    pub model: Option<&'a str>,
    #[serde(default)]
    pub onendtouch: Option<&'a str>,
    #[serde(default)]
    pub onstarttouch: Option<&'a str>,
    #[serde(default)]
    pub ontrigger: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub parentname: Option<&'a str>,
    pub spawnflags: u32,
    #[serde(default)]
    pub speed: Option<u16>,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
    #[serde(default)]
    pub target: Option<&'a str>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerVphysicsMotion<'a> {
    pub model: &'a str,
    pub origin: Vector,
    pub particletrailendsize: u8,
    pub particletraillifetime: u8,
    pub particletrailmaterial: &'a str,
    pub particletrailstartsize: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    pub setadditionalairdensity: bool,
    pub setangvelocitylimit: f32,
    pub setangvelocityscale: f32,
    pub setgravityscale: f32,
    pub setlinearforce: f32,
    pub setlinearforceangles: Angles,
    pub setvelocitylimit: f32,
    pub setvelocitylimitdelta: f32,
    pub setvelocityscale: f32,
    pub spawnflags: u32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct TriggerWind<'a> {
    pub angles: Angles,
    pub directionnoise: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    pub holdnoise: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub holdtime: bool,
    pub model: &'a str,
    pub origin: Vector,
    pub spawnflags: u32,
    pub speed: u16,
    pub speednoise: u8,
    #[serde(deserialize_with = "deserialize_bool")]
    pub startdisabled: bool,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WaterLodControl<'a> {
    pub cheapwaterenddistance: f32,
    pub cheapwaterstartdistance: f32,
    #[serde(default)]
    pub origin: Option<Vector>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAk47<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub ammo: Option<u16>,
    pub angles: Angles,
    #[serde(default)]
    pub onplayerpickup: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponAwp<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub ammo: Option<u16>,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponDeagle<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub ammo: Option<u16>,
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponElite<'a> {
    pub ammo: u16,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFamas {
    pub ammo: u16,
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub shadowcastdist: bool,
    pub spawnflags: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFiveseven<'a> {
    pub ammo: u16,
    pub angles: Angles,
    pub origin: Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponFlashbang<'a> {
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponG3sg1 {
    pub ammo: u16,
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub shadowcastdist: bool,
    pub spawnflags: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponGlock<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub ammo: u16,
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponHegrenade<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponKnife<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    pub angles: Angles,
    #[serde(default)]
    pub onplayerpickup: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM249<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub ammo: Option<u16>,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM3<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub ammo: Option<u16>,
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponM4a1<'a> {
    #[serde(default)]
    pub ammo: Option<u16>,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub shadowcastdist: bool,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponMac10 {
    pub ammo: u16,
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub shadowcastdist: bool,
    pub spawnflags: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponP228<'a> {
    pub ammo: u16,
    pub angles: Angles,
    pub origin: Vector,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponP90<'a> {
    #[serde(default)]
    pub ammo: Option<u16>,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub shadowcastdist: bool,
    pub spawnflags: u32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponScout<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub ammo: Option<u16>,
    pub angles: Angles,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disablereceiveshadows: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub disableshadows: bool,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(default)]
    pub onplayerpickup: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(default)]
    pub renderfx: Option<u8>,
    #[serde(default)]
    pub rendermode: Option<u8>,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSg550 {
    pub ammo: u16,
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub shadowcastdist: bool,
    pub spawnflags: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponSmokegrenade<'a> {
    pub angles: Angles,
    pub origin: Vector,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponTmp {
    pub ammo: u16,
    pub angles: Angles,
    pub fademaxdist: f32,
    pub fademindist: f32,
    pub fadescale: f32,
    #[serde(deserialize_with = "deserialize_bool")]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub nodamageforces: bool,
    pub origin: Vector,
    pub renderamt: u8,
    pub rendercolor: Color,
    #[serde(deserialize_with = "deserialize_bool")]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    pub shadowcastdist: bool,
    pub spawnflags: u32,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponUmp45<'a> {
    pub ammo: u16,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub shadowcastdist: bool,
    pub spawnflags: u32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponUsp<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub ammo: Option<u16>,
    pub angles: Angles,
    #[serde(default)]
    pub onplayerpickup: Option<&'a str>,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    pub spawnflags: u32,
    pub targetname: &'a str,
}
#[derive(Debug, Clone, Deserialize)]
pub struct WeaponXm1014<'a> {
    pub ammo: u32,
    pub angles: Angles,
    #[serde(default)]
    pub fademaxdist: Option<f32>,
    #[serde(default)]
    pub fademindist: Option<f32>,
    #[serde(default)]
    pub fadescale: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub maxdxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub mindxlevel: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub nodamageforces: bool,
    pub origin: Vector,
    #[serde(default)]
    pub renderamt: Option<u8>,
    #[serde(default)]
    pub rendercolor: Option<Color>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub renderfx: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub rendermode: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub shadowcastdist: bool,
    pub spawnflags: u32,
    #[serde(default)]
    pub targetname: Option<&'a str>,
}
#[derive(Debug, Clone, Deserialize)]
pub struct Worldspawn<'a> {
    #[serde(default)]
    pub _minlight: Option<f32>,
    #[serde(default)]
    pub chaptertitle: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub coldworld: bool,
    #[serde(default)]
    pub comment: Option<&'a str>,
    #[serde(default)]
    pub csg_options: Option<&'a str>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub defaultteam: bool,
    #[serde(default)]
    pub detailmaterial: Option<&'a str>,
    #[serde(default)]
    pub detailvbsp: Option<&'a str>,
    #[serde(default)]
    pub fogcolor: Option<Color>,
    #[serde(default)]
    pub fogcolor2: Option<Color>,
    #[serde(default)]
    pub fogdir: Option<Angles>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub fogenable: bool,
    #[serde(default)]
    pub fogend: Option<f32>,
    #[serde(default)]
    pub fogstart: Option<u16>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub gametitle: bool,
    #[serde(default)]
    pub light: Option<u8>,
    #[serde(default)]
    pub mapversion: Option<u16>,
    #[serde(default)]
    pub maxoccludeearea: Option<f32>,
    pub maxpropscreenwidth: f32,
    #[serde(default)]
    pub maxrange: Option<u16>,
    #[serde(default)]
    pub message: Option<&'a str>,
    #[serde(default)]
    pub minoccluderarea: Option<f32>,
    #[serde(default)]
    pub minpropscreenwidth: Option<f32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub newunit: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub no_decomp: bool,
    pub skyname: &'a str,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub sounds: bool,
    #[serde(default)]
    pub spawnflags: Option<u32>,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub startdark: bool,
    #[serde(default)]
    pub waveheight: Option<f32>,
    pub world_maxs: Vector,
    pub world_mins: Vector,
    #[serde(deserialize_with = "deserialize_bool")]
    #[serde(default)]
    pub worldtype: bool,
}
