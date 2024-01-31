
use super::*;

#[acmd_script( agent = "richter", script = "game_attackairn" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 43, 80, 0, 45, 4.5, 0.0, 10.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 43, 85, 0, 60, 4.4, 0.0, 12.3, 7.0, Some(0.0), Some(6.5), Some(18.1), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 43, 85, 0, 60, 4.4, 0.0, 11.0, 7.5, Some(0.0), Some(5.3), Some(19.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 43, 85, 0, 60, 4.3, 0.0, 10.0, 7.0, Some(0.0), Some(5.3), Some(16.7), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 43, 85, 0, 45, 4.3, 0.0, 11.2, 1.0, Some(0.0), Some(5.3), Some(12.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 43, 85, 0, 45, 4.3, 0.0, 13.0, -1.0, Some(0.0), Some(5.7), Some(10.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 43, 85, 0, 45, 4.3, 0.0, 15.5, -5.0, Some(0.0), Some(6.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 43, 80, 0, 45, 4.3, 0.0, 16.5, -7.0, Some(0.0), Some(8.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 43, 80, 0, 60, 4.5, 0.0, 10.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 43, 80, 0, 45, 4.2, 0.0, 18.0, -7.5, Some(0.0), Some(9.3), Some(3.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 43, 80, 0, 45, 4.2, 0.0, 20.5, -8.5, Some(0.0), Some(11.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 43, 80, 0, 45, 4.2, 0.0, 21.0, -10.9, Some(0.0), Some(11.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 34.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_attack100_end"), Hash40::new("top"), 0, 12, 8.1, 0, -32, 110, 1.2, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.3);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_attack100_end"), Hash40::new("top"), 0, 16.6, -3.3, 0, 213.7, -95, 0.9, true);
        LAST_EFFECT_SET_ALPHA(fighter, 0.2);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_attack100_end"), false, true);
    }
}

#[acmd_script( agent = "richter", script = "sound_attackairn" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_air_n_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_whip_holding"));
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        PLAY_SEQUENCE(fighter, Hash40::new("seq_richter_rnd_attack"));
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_swing_m"));
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_richter_swing_s"));
    }
}

#[acmd_script( agent = "richter_whip", script = "effect_attackairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_whip_attack_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot27"), 1, 0, 0, 0, 0, 0, 1.5, true);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot23"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot18"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot14"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot4"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 0.5);
    }
}

#[acmd_script( agent = "richter_whip", script = "game_landingairn" , category = ACMD_GAME , low_priority)]
unsafe fn richter_landing_air_n_game(fighter: &mut L2CAgentBase) {
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter", script = "game_attackairfhi" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_f_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 62, 0, 84, 4.5, 0.0, 23.0, 38.0, Some(0.0), Some(22.37097), Some(36.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 120, 38, 0, 80, 3.8, 0.0, 21.0, 34.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 361, 40, 0, 58, 2.3, 0.0, 10.0, 7.0, Some(0.0), Some(20.0), Some(32.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 50, 0, 50, 5.5, 0.0, 10.0, 5.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackairfhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_air_f_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.141, 0.851, 1.0);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(fighter, 3.0);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_aura"), false, true);
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attackairfhi" , category = ACMD_GAME , low_priority)]
unsafe fn richter_whip_attack_air_f_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

#[acmd_script( agent = "richter_whip", script = "effect_attackairfhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_whip_attack_air_f_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("hookshot7"), -8, 0, 0, 0, -40, -90, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot9"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot26"), 2, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot4"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_whip_straight"), true, true);
    }
}

#[acmd_script( agent = "richter", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 62, 0, 84, 4.5, 0.0, 8.0, 40.0, Some(0.0), Some(8.0), Some(38.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 110, 40, 0, 85, 3.8, 0.0, 8.0, 36.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 361, 40, 0, 58, 2.3, 0.0, 8.0, 7.0, Some(0.0), Some(8.0), Some(34.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 50, 0, 50, 5.5, 0.0, 8.0, 5.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.141, 0.851, 1.0);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(fighter, 3.0);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_aura"), false, true);
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attackairf" , category = ACMD_GAME , low_priority)]
unsafe fn richter_whip_attack_air_f_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

#[acmd_script( agent = "richter_whip", script = "effect_attackairf" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_whip_attack_air_f_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("hookshot7"), -8, 0, 0, 0, -75, -90, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot9"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot14"), 0, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot4"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_whip_straight"), true, true);
    }
}

#[acmd_script( agent = "richter", script = "game_attackairflw" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_f_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        //ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 361, 62, 0, 84, 4.5, 0.0, -7.0, 38.0, Some(0.0), Some(-6.37097), Some(36.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 120, 38, 0, 80, 3.8, 0.0, -5.4, 34.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_aura"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 361, 40, 0, 58, 2.3, 0.0, 6.0, 7.0, Some(0.0), Some(-5.0), Some(32.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_whip"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_RICHTER_WHIP, *ATTACK_REGION_WHIP);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 361, 50, 0, 50, 5.5, 0.0, 6.0, 5.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 31.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackairflw" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_air_f_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_bottle_appear"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.3, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.141, 0.851, 1.0);
        LAST_EFFECT_SET_RATE(fighter, 2.5);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_aura"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.35, true);
        LAST_EFFECT_SET_RATE(fighter, 3.0);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_damage_aura"), false, true);
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attackairflw" , category = ACMD_GAME , low_priority)]
unsafe fn richter_whip_attack_air_f_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

#[acmd_script( agent = "richter_whip", script = "effect_attackairflw" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_whip_attack_air_f_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_straight"), Hash40::new("hookshot7"), -8, 0, 0, 0, -130, -90, 0.7, true);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot9"), 0, 0, 0, 0, 0, 0, 1, true);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_flash_top"), Hash40::new("hookshot26"), 2, 0, 0, 0, 0, 0, 1.5, true);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 2);
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot4"), 0, 0, 0, 0, 0, 0, 1.2, false, 0.65);
        LAST_EFFECT_SET_RATE(fighter, 2);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("richter_whip_straight"), true, true);
    }
}

#[acmd_script( agent = "richter_whip", script = "game_landingairf" , category = ACMD_GAME , low_priority)]
unsafe fn richter_landing_air_f_game(fighter: &mut L2CAgentBase) {
    let boma = fighter.boma();
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter", script = "game_attackairbhi" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_b_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.5, 361, 89, 0, 35, 3.0, 0.0, 12.4, -5.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.5, 361, 89, 0, 35, 3.5, 0.0, 10.0, -8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 361, 89, 0, 35, 4.5, 0.0, 7.9, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 89, 0, 30, 2.5, 0.0, 12.4, -5.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 89, 0, 30, 3.0, 0.0, 10.0, -8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 45, 89, 0, 30, 3.5, 0.0, 7.9, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackairbhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_air_b_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 12, -5.7, -216, 9, 0, 0.5, true, 0.8);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
        LAST_EFFECT_SET_COLOR(fighter, 0.769, 0.769, 0.769);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 7.4, -13.9, 0, 0, 0, 1.0, true, 0.9);
        LAST_EFFECT_SET_COLOR(fighter, 0.769, 0.769, 0.769);
    }
}

#[acmd_script( agent = "richter", script = "sound_attackairbhi" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_air_b_hi_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack05"));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_l"));
        PLAY_SE(fighter, Hash40::new("se_richter_appeal_s02"));
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attackairbhi" , category = ACMD_GAME , low_priority)]
unsafe fn richter_whip_attack_air_b_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

#[acmd_script( agent = "richter_whip", script = "effect_attackairbhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_whip_attack_air_b_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1, false, 0.65);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

#[acmd_script( agent = "richter", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.5, 361, 89, 0, 35, 3.0, 0.0, 12.4, -5.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.5, 361, 89, 0, 35, 3.5, 0.0, 10.0, -8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 361, 89, 0, 35, 4.5, 0.0, 7.9, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 89, 0, 30, 2.5, 0.0, 12.4, -5.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 89, 0, 30, 3.0, 0.0, 10.0, -8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 45, 89, 0, 30, 3.5, 0.0, 7.9, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    } 
}

#[acmd_script( agent = "richter", script = "effect_attackairb" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 12, -5.7, -216, 9, 0, 0.5, true, 0.8);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
        LAST_EFFECT_SET_COLOR(fighter, 0.769, 0.769, 0.769);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 7.4, -13.9, 0, 0, 0, 1.0, true, 0.9);
        LAST_EFFECT_SET_COLOR(fighter, 0.769, 0.769, 0.769);
    }
}

#[acmd_script( agent = "richter", script = "sound_attackairb" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_air_b_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack05"));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_l"));
        PLAY_SE(fighter, Hash40::new("se_richter_appeal_s02"));
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attackairb" , category = ACMD_GAME , low_priority)]
unsafe fn richter_whip_attack_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

#[acmd_script( agent = "richter_whip", script = "effect_attackairb" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_whip_attack_air_b_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1, false, 0.65);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

#[acmd_script( agent = "richter", script = "game_attackairblw" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_b_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
		ATTACK(fighter, 0, 0, Hash40::new("top"), 12.5, 361, 89, 0, 35, 3.0, 0.0, 12.4, -5.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.5, 361, 89, 0, 35, 3.5, 0.0, 10.0, -8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 14.0, 361, 89, 0, 35, 4.5, 0.0, 7.9, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 45, 89, 0, 30, 2.5, 0.0, 12.4, -5.6, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 45, 89, 0, 30, 3.0, 0.0, 10.0, -8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 11.0, 45, 89, 0, 30, 3.5, 0.0, 7.9, -14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
		HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 48.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackairblw" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_air_b_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_speedline"), Hash40::new("top"), -1, 12, -5.7, -216, 9, 0, 0.5, true, 0.8);
        LAST_EFFECT_SET_RATE(fighter, 1.0);
        LAST_EFFECT_SET_COLOR(fighter, 0.769, 0.769, 0.769);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_attack_impact"), Hash40::new("top"), 0, 7.4, -13.9, 0, 0, 0, 1.0, true, 0.9);
        LAST_EFFECT_SET_COLOR(fighter, 0.769, 0.769, 0.769);
    }
}

#[acmd_script( agent = "richter", script = "sound_attackairblw" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_air_b_lw_sound(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("vc_richter_attack05"));
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        PLAY_SE(fighter, Hash40::new("se_common_punch_kick_swing_l"));
        PLAY_SE(fighter, Hash40::new("se_richter_appeal_s02"));
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attackairblw" , category = ACMD_GAME , low_priority)]
unsafe fn richter_whip_attack_air_b_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

#[acmd_script( agent = "richter_whip", script = "effect_attackairblw" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_whip_attack_air_b_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("richter_whip_light"), Hash40::new("hookshot10"), 0, 0, 0, 0, 0, 0, 1, false, 0.65);
        EFFECT_FOLLOW(fighter, Hash40::new("richter_whip_light_s"), Hash40::new("hookshot3"), 0, 0, 0, 0, 0, 0, 1, true);
    }
}

#[acmd_script( agent = "richter_whip", script = "game_landingairb" , category = ACMD_GAME , low_priority)]
unsafe fn richter_landing_air_b_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        PhysicsModule::set_2nd_status(boma, *PH2NDARY_CRAW_NONE);
    }
}

#[acmd_script( agent = "richter", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 8.0, 75, 100, 0, 40, 4.4, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.0, 75, 100, 0, 40, 6.0, 3.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "richter", script = "effect_attackairhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_arc_c"), Hash40::new("top"), 0, 12.6, 2.5, 0, 75, 82, 0.86, true);
        LAST_EFFECT_SET_COLOR(fighter, 0.902, 0.784, 0.333);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_c"), false, true);
    }
}

#[acmd_script( agent = "richter", script = "sound_attackairhi" , category = ACMD_SOUND , low_priority)]
unsafe fn richter_attack_air_hi_sound(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    frame(lua_state, 8.0);
    if is_excute(agent) {
        PLAY_SE(agent, Hash40::new("se_common_punch_kick_swing_m"));
        PLAY_SEQUENCE(agent, Hash40::new("seq_richter_rnd_attack"));
    }
}

#[acmd_script( agent = "richter_whip", script = "game_attackairhi" , category = ACMD_GAME , low_priority)]
unsafe fn richter_whip_attack_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma(); 
}

#[acmd_script( agent = "richter_whip", script = "effect_attackairhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn richter_whip_attack_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
}

#[acmd_script( agent = "richter", script = "game_attackairlw" , category = ACMD_GAME , low_priority)]
unsafe fn richter_attack_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        //WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        //SET_SPEED_EX(fighter, 0, 0.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        //WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        //KineticModule::suspend_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        //WorkModule::on_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        //FT_MOTION_RATE(fighter, 0.417);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        SET_SPEED_EX(fighter, 1.6, -3.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(boma, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        JostleModule::set_status(boma, false);
    }
    frame(lua_state, 16.0);
    if is_excute(fighter) {
        /* Ground-only */
        ATTACK(fighter, 0, 0, Hash40::new("top"), 12.0, 280, 85, 0, 35, 5.5, 0.0, 1.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        /* Air-only */
        ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 280, 53, 0, 35, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 1, false);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 72, 55, 0, 88, 6.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        JostleModule::set_status(boma, true);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::off_flag(boma, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

pub fn install() {
    install_acmd_scripts!(
        richter_attack_air_n_game,
        richter_attack_air_n_effect,
        richter_attack_air_n_sound,
        richter_whip_attack_air_n_effect,
        richter_landing_air_n_game,

        richter_attack_air_f_hi_game,
        richter_attack_air_f_hi_effect,
        richter_whip_attack_air_f_hi_game,
        richter_whip_attack_air_f_hi_effect,
        richter_attack_air_f_game,
        richter_attack_air_f_effect,
        richter_whip_attack_air_f_game,
        richter_whip_attack_air_f_effect,
        richter_attack_air_f_lw_game,
        richter_attack_air_f_lw_effect,
        richter_whip_attack_air_f_lw_game,
        richter_whip_attack_air_f_lw_effect,
        richter_landing_air_f_game,

        richter_attack_air_b_hi_game,
        richter_attack_air_b_hi_effect,
        richter_attack_air_b_hi_sound,
        richter_whip_attack_air_b_hi_game,
        richter_whip_attack_air_b_hi_effect,
        richter_attack_air_b_game,
        richter_attack_air_b_effect,
        richter_attack_air_b_sound,
        richter_whip_attack_air_b_game,
        richter_whip_attack_air_b_effect,
        richter_attack_air_b_lw_game,
        richter_attack_air_b_lw_effect,
        richter_attack_air_b_lw_sound,
        richter_whip_attack_air_b_lw_game,
        richter_whip_attack_air_b_lw_effect,
        richter_landing_air_b_game,

        richter_attack_air_hi_game,
        richter_attack_air_hi_effect,
        richter_whip_attack_air_hi_effect,
        richter_attack_air_hi_sound,
        richter_whip_attack_air_hi_game,

        richter_attack_air_lw_game,
    );
}

