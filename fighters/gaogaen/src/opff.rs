// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 // Handles double jump reset, cancel at the apex, and early activation of the dive
unsafe fn cross_chop_techniques(fighter: &mut L2CFighterCommon) {
    
    if fighter.is_motion_one_of(&[Hash40::new("special_hi"), Hash40::new("special_air_hi_start")]) {
        // Detect if you've hit the initial portion of the move so we know whether or not to lower the damage of the falling portion
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT){
            VarModule::on_flag(fighter.object(), vars::gaogaen::IS_HIT_SPECIAL_HI_RISE);
        }
        // Cancel into the dive if holding the special button during the aerial version
        /*
        if fighter.is_motion(Hash40::new("special_air_hi_start")){
            if MotionModule::frame(fighter.module_accessor) > 14.0 && MotionModule::frame(fighter.module_accessor) < 16.0 {
                if fighter.is_button_on(Buttons::Special) || fighter.is_button_on(Buttons::SpecialRaw) {
                    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
                    KineticModule::clear_speed_all(fighter.module_accessor);
                    fighter.change_status_req(*FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_TURN, false); 
                    return;
                }
            }
        }
        */
        if MotionModule::frame(fighter.module_accessor) > 20.0 && !VarModule::is_flag(fighter.object(), vars::gaogaen::SHOULD_CROSS_CHOP_DIVE_EARLY) { // Not checking for the SHOULD_CROSS_CHOP_DIVE_EARLY flag has interesting behavior if holding special and shield, allows him to do a little aerial flip that stalls him in the air a bit
            if fighter.is_button_on(Buttons::Guard) {
                //DamageModule::add_damage(fighter.module_accessor, 1.0, 0);
                VarModule::on_flag(fighter.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL);
            }    
        }
    }
    // Cancel out at the apex if the shield input was detected
    /*
    if fighter.is_motion(Hash40::new("special_air_hi_turn")) {
        if MotionModule::frame(fighter.module_accessor) > 7.0 {
            if VarModule::is_flag(fighter.object(), vars::gaogaen::IS_INPUT_CROSS_CHOP_CANCEL){
                VarModule::on_flag(fighter.object(), vars::common::UP_SPECIAL_CANCEL);
                fighter.change_status_req(*FIGHTER_STATUS_KIND_FALL, true);
                //return;
            }
        }
    }
    */
    if fighter.is_status(*FIGHTER_GAOGAEN_STATUS_KIND_SPECIAL_HI_FALL) {
        if fighter.get_num_used_jumps() == fighter.get_jump_count_max() {
            WorkModule::set_int(fighter.module_accessor, fighter.get_jump_count_max() - 1, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
}

unsafe fn revenge_command_dash(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("special_lw_start")){
        if VarModule::is_flag(fighter.object(), vars::gaogaen::IS_SPECIAL_LW_COMMAND_DASH){
            if StopModule::is_damage(fighter.module_accessor){
                VarModule::on_flag(fighter.object(), vars::gaogaen::DID_SPECIAL_LW_COMMAND_DASH_TANK_DAMAGE);
                DamageModule::reset_no_reaction_mode_status(fighter.module_accessor);
            }
            if VarModule::is_flag(fighter.object(), vars::gaogaen::IS_ENABLE_SPECIAL_LW_LARIAT_INPUT){
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL_RAW){
                    VarModule::on_flag(fighter.object(), vars::gaogaen::IS_INPUT_SPECIAL_LW_LARIAT);
                }
            }
        }
    }
}

// Incineroar Fthrow Movement
unsafe fn fthrow_movement(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_STATUS_KIND_THROW) 
     && fighter.is_motion(smash::phx::Hash40::new("throw_f"))
     && fighter.is_situation(*SITUATION_KIND_GROUND) 
     && fighter.stick_x() != 0.0 {

        let motion_vec = x_motion_vec(1.0, fighter.stick_x());
        KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &motion_vec);
        
    }
}

unsafe fn rotate_bust(boma: &mut BattleObjectModuleAccessor, max_angle: f32, min_angle: f32, strength: f32) {
    let mut angle = min_angle.abs();
    if strength > 0.0 {
        angle = max_angle
    }
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: ((angle * -1.0 * strength) - 2.5)};

    // this has to be called every frame, or you snap back to the normal joint angle
    ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
}

// boma: its a boma 
// lean_frame: frame to interpolate to the intended angle amount until
// return_frame: frame to start interpolating back to regular angle
// max_angle: maximum angle you can lean upwards, in degrees
// min_angle: minimum angle that we should be able to rotate downwards, in degrees
unsafe fn catch_lean(boma: &mut BattleObjectModuleAccessor, lean_frame: f32, return_frame: f32, max_angle: f32, min_angle: f32) {
    let stick_y = ControlModule::get_stick_y(boma);
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let grab_y = VarModule::get_float(boma.object(), vars::gaogaen::ANGLE_GRAB_STICK_Y);
    if frame >= 0.0 && frame < lean_frame {
        // linear interpolate to stick position,
        // while getting stick position still
        VarModule::set_float(boma.object(), vars::gaogaen::ANGLE_GRAB_STICK_Y, stick_y);
        rotate_bust(boma, max_angle, min_angle, stick_y * ((frame as f32) / 7.0));
    } else if frame >= lean_frame && frame < return_frame {
        // rotate at selected angle for each frame
        rotate_bust(boma, max_angle, min_angle, grab_y);
    } else {
        // linear interpolate back to normal
        rotate_bust(boma, max_angle, min_angle, grab_y * (1.0 - ((frame - return_frame) / (end_frame - return_frame))));
    }
}

unsafe fn angled_grab(fighter: &mut L2CFighterCommon) {

    if fighter.is_status(*FIGHTER_STATUS_KIND_CATCH) {
        catch_lean(fighter.boma(), 7.0, 30.0, 50.0, 30.0);
    } else if fighter.is_status(*FIGHTER_STATUS_KIND_CATCH_TURN) {
        catch_lean(fighter.boma(), 12.0, 30.0, 30.0, 15.0);
    } else if  fighter.is_status(*FIGHTER_STATUS_KIND_CATCH_DASH) {
        catch_lean(fighter.boma(), 11.0, 30.0, 30.0, 15.0);
    }
}


// boma: its a boma
// start_frame: frame to start interpolating the waist rotation
// bend_frame: frame to interpolate to the intended angle amount until
// return_frame: frame to start interpolating back to regular angle
// straight_frame: frame the waist should be at the regular angle again
unsafe fn command_grab_joint_rotate(boma: &mut BattleObjectModuleAccessor, rotation_amount: f32, start_frame: f32, bend_frame: f32, return_frame: f32, straight_frame: f32) {
    let frame = MotionModule::frame(boma);
    let end_frame = MotionModule::end_frame(boma);
    let max_z_rotation = rotation_amount;
    let mut rotation = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        
    if frame >= start_frame && frame < return_frame {
        // this has to be called every frame, or you snap back to the normal joint angle
        // interpolate to the respective waist bend angle
        let calc_z_rotate = max_z_rotation * (frame / (bend_frame - start_frame));
        let mut z_rotation = 0.0;
        if max_z_rotation < 0.0 {
            z_rotation = calc_z_rotate.clamp(max_z_rotation, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_z_rotation);
        }
        rotation = Vector3f{x: 0.0, y: 0.0, z: z_rotation};
        ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    } else if frame >= return_frame && frame < straight_frame {
        // linear interpolate back to normal
        let calc_z_rotate = max_z_rotation *(1.0 - (frame - return_frame) / (straight_frame - return_frame));
        let mut z_rotation = 0.0;
        if max_z_rotation < 0.0 {
            z_rotation = calc_z_rotate.clamp(max_z_rotation, 0.0);
        }
        else{
            z_rotation = calc_z_rotate.clamp(0.0, max_z_rotation);
        }
        rotation = Vector3f{x: 0.0, y: 0.0, z: z_rotation};
        ModelModule::set_joint_rotate(boma, Hash40::new("waist"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8})
    }
}

unsafe fn alolan_whip_special_grabs(fighter: &mut L2CFighterCommon) {
    if fighter.is_motion(Hash40::new("special_s_start")){
        if VarModule::is_flag(fighter.object(), vars::gaogaen::IS_SPECIAL_S_ALTERNATE_GRAB) {
            // OTG Grab
            if VarModule::is_flag(fighter.object(), vars::gaogaen::IS_SPECIAL_S_GROUND_GRAB){
                command_grab_joint_rotate(fighter.boma(), 20.0, 13.0, 18.0, 30.0, 45.0);
            }
            // Anti-air grab
            else if VarModule::is_flag(fighter.object(), vars::gaogaen::IS_SPECIAL_S_AIR_GRAB){
                command_grab_joint_rotate(fighter.boma(), -50.0, 13.0, 18.0, 30.0, 45.0);
            }
        }
    }
}

#[utils::macros::opff(FIGHTER_KIND_GAOGAEN )]
pub fn gaogaen_opff(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		cross_chop_techniques(fighter);
        revenge_command_dash(fighter);
        fthrow_movement(fighter);
        angled_grab(fighter); 
        alolan_whip_special_grabs(fighter);
    }
}
