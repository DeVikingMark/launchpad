use crate::common_setup::contract_boxes::{contract_open_edition_factory, custom_mock_app};
use crate::common_setup::setup_minter::open_edition_minter::mock_params::mock_params_custom;
use cosmwasm_std::Addr;
use cw_multi_test::Executor;
use open_edition_factory::helpers::FactoryContract;
use open_edition_factory::msg::InstantiateMsg;
use sg_multi_test::StargazeApp;

const GOVERNANCE: &str = "governance";

pub fn proper_instantiate() -> (StargazeApp, FactoryContract) {
    let mut app = custom_mock_app();
    let factory_id = app.store_code(contract_open_edition_factory());
    let minter_id = 2;

    let mut params = mock_params_custom(None, None, None);
    params.code_id = minter_id;

    let factory_contract_addr = app
        .instantiate_contract(
            factory_id,
            Addr::unchecked(GOVERNANCE),
            &InstantiateMsg { params },
            &[],
            "factory",
            None,
        )
        .unwrap();

    (app, FactoryContract(factory_contract_addr))
}
