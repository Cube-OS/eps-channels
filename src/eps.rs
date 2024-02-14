use cubeos_service::*;
use isis_eps_api::*;
use std::str::FromStr;

app_macro! {
    isis_eps_service: Eps { 
        query: EpsPing => fn eps_ping(&self) -> Result<()>; out: ();
        query: Status => fn system_status(&self) -> Result<SystemStatus>; out: SystemStatus;
        query: OvercurrentState => fn overcurrent_state(&self) -> Result<OverCurrentFaultState>; out: OverCurrentFaultState;
        // query: AbfState => fn abf_state(&self) -> Result<ABFState>; out: ABFState;
        // query: PduHk => fn pdu_hk(&self, mode: PDUHkSel) -> Result<PDUHk>; out: PDUHk;
        // query: PbuHk => fn pbu_hk(&self, mode: PBUHkSel) -> Result<PBUHk>; out: PBUHk;
        query: PiuHk => fn piu_hk(&self, mode: PIUHkSel) -> Result<PIUHk>; out: PIUHk;
        // query: PcuHk => fn pcu_hk(&self, mode: PCUHkSel) -> Result<PCUHk>; out: PCUHk;
        query: GetConfigParamWrite => fn get_config_para_write(&self, param: ConfigParamWrite) -> EpsResult<Output>; out: Output;
        query: GetConfigParamRead => fn get_config_para_read(&self, param: ConfigParamRead) -> EpsResult<Output>; out: Output;
        query: SetConfigParamU32 => fn set_config_para_u32(&self, param: ConfigParamWriteU32, value: u32) -> EpsResult<Output>; out: Output;
        query: SetConfigParamU16 => fn set_config_para_u16(&self, param: ConfigParamWriteU16, value: u16) -> EpsResult<Output>; out: Output;
        query: SetConfigParamU8 => fn set_config_para_u8(&self, param: ConfigParamWriteU8, value: u8) -> EpsResult<Output>; out: Output;
        query: SetConfigParamI16 => fn set_config_para_i16(&self, param: ConfigParamWriteI16, value: i16) -> EpsResult<Output>; out: Output;
        query: SetConfigParamI8 => fn set_config_para_i8(&self, param: ConfigParamWriteI8, value: i8) -> EpsResult<Output>; out: Output;
        query: ResetConfigParam => fn reset_param(&self, param: ConfigParamWrite) -> EpsResult<Output>; out: Output;
        mutation: SysReset => fn sys_reset(&self, ret_key: u8) -> Result<()>;
        mutation: ShutDownAll =>fn shutdown_all(&self) -> EpsResult<()>;
        mutation: WatchdogReset => fn watchdog_reset(&self) -> EpsResult<()>;
        mutation: SetGroupState => fn set_group_state(&self, typ_group: BusGroup, state: BusChannelState) -> EpsResult<()>;
        mutation: SetGroupOutputs => fn set_group_outputs(&self, typ_group: BusGroup, channels: Vec<u8>) -> EpsResult<()>;
        mutation: SetSingleOutput => fn set_single_output(&self, typ_channel: BusChannel, eps_ch_idx: u8) -> EpsResult<()>;
        mutation: ModeSwitchFn => fn mode_switch(&self, mode: ModeSwitch) -> EpsResult<()>;
        mutation: CorrectTime => fn correct_time(&self, time_correction: i32) -> EpsResult<()>;
        mutation: ResetAllCounters => fn reset_all_counters(&self) -> EpsResult<()>;        
        mutation: ResetAllConf => fn reset_all_conf(&self) -> EpsResult<()>;
        mutation: LoadConfig => fn load_config(&self) -> EpsResult<()>;
        mutation: SaveConfig => fn save_config(&self) -> EpsResult<()>;
        mutation: ForceSaveConfig => fn force_save_config(&self) -> EpsResult<()>;
    }
}