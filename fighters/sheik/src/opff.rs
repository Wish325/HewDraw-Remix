// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn bouncing_fish_return_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN && frame <= 15.0 {
        if situation_kind == *SITUATION_KIND_AIR {
            boma.check_jump_cancel(false);
        }
    }
}

unsafe fn nspecial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
            }
        }
    }
}


extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

pub unsafe fn teleport_wall_ride(boma: &mut BattleObjectModuleAccessor, status_kind: i32, id: usize) {
    /*
    if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE {
        if compare_mask(ControlModule::get_pad_flag(boma), *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER) {
            StatusModule::change_status_request_from_script(boma, *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END, false);
        }
    }
    */

    // Wall Ride momentum fixes
    let mut wall_ride = Vector3f{x: 1.0, y: 1.0, z: 1.0};
    let touch_right = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
    let touch_left = GroundModule::is_wall_touch_line(boma, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
    let warp_speed = WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("warp_speed_add")) + WorkModule::get_param_float(boma, hash40("param_special_hi"), hash40("warp_speed_mul"));

    if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_MOVE {
        if touch_right || touch_left || VarModule::is_flag(boma.object(), vars::common::instance::IS_TELEPORT_WALL_RIDE) {
            VarModule::on_flag(boma.object(), vars::common::instance::IS_TELEPORT_WALL_RIDE);
            if (touch_right && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0) || (touch_left && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0) {
                let rise_speed = KineticModule::get_sum_speed_y(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                if rise_speed > 0.0 {
                    wall_ride = Vector3f{x: 0.0, y: (warp_speed / rise_speed), z: 1.0};
                }
                else {
                    wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                }
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    else if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END {
        if touch_right || touch_left {
            if (touch_right && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) < 0.0) || (touch_left && KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) > 0.0) {
                wall_ride = Vector3f{x: 0.0, y: 1.0, z: 1.0};
                KineticModule::mul_speed(boma, &wall_ride, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
            }
        }
    }
    else {
        VarModule::off_flag(boma.object(), vars::common::instance::IS_TELEPORT_WALL_RIDE);
    }
}

// pub unsafe fn hitfall_aerials(fighter: &mut L2CFighterCommon, frame: f32) {
//     if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) {
//         // only allow the last hit of uair to be hitfalled
//         if fighter.is_motion(Hash40::new("attack_air_hi")) {
//             if frame >= 23.0 && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
//                 fighter.check_hitfall();
//             }
//         }
//         else {
//             fighter.check_hitfall();
//         }
//     }
// }

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bouncing_fish_return_cancel(fighter, boma, status_kind, situation_kind, cat[0], frame);
    nspecial_cancels(fighter, boma, status_kind, situation_kind);
    teleport_wall_ride(boma, status_kind, id);
    //hitfall_aerials(fighter, frame);
}

#[utils::macros::opff(FIGHTER_KIND_SHEIK )]
pub fn sheik_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		sheik_frame(fighter)
    }
}

pub unsafe fn sheik_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}