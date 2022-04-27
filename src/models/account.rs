use serde::{Deserialize, Serialize};

use super::{
    AccountAddress, AccountDataset, AccountHolder, AccountProfile, AutoRefresh, BankTransferCode,
    Coverage, FullAccountNumberList, LoanPayoffDetails, Money, PaymentProfile, RewardBalance,
};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub available_cash: Option<Money>,
    pub include_in_net_worth: Option<bool>,
    pub money_market_balance: Option<Money>,
    pub enrollment_date: Option<String>,
    pub estimated_date: Option<String>,
    pub memo: Option<String>,
    pub guarantor: Option<String>,
    pub interest_paid_last_year: Option<Money>,
    pub last_updated: Option<String>,
    pub balance: Option<Money>,
    pub home_insurance_type: Option<String>,
    pub id: Option<i64>,
    pub cash: Option<Money>,
    pub total_credit_line: Option<Money>,
    pub provider_name: Option<String>,
    pub valuation_type: Option<String>,
    pub margin_balance: Option<Money>,
    pub apr: Option<f64>,
    pub available_credit: Option<Money>,
    pub current_balance: Option<Money>,
    pub is_manual: Option<bool>,
    pub profile: Option<AccountProfile>,
    pub escrow_balance: Option<Money>,
    pub next_level: Option<String>,
    pub classification: Option<String>,
    pub loan_payoff_amount: Option<Money>,
    pub interest_rate_type: Option<String>,
    pub loan_pay_by_date: Option<String>,
    pub face_amount: Option<Money>,
    pub policy_from_date: Option<String>,
    pub premium_payment_term: Option<String>,
    pub policy_term: Option<String>,
    pub repayment_plan_type: Option<String>,
    pub available_balance: Option<Money>,
    pub account_status: Option<String>,
    pub life_insurance_type: Option<String>,
    pub full_account_number: Option<String>,
    pub premium: Option<Money>,
    pub aggregation_source: Option<String>,
    pub over_draft_limit: Option<Money>,
    pub nickname: Option<String>,
    pub term: Option<String>,
    pub interest_rate: Option<f64>,
    pub death_benefit: Option<Money>,
    pub address: Option<AccountAddress>,
    pub cash_value: Option<Money>,
    pub holder: Option<Vec<AccountHolder>>,
    pub var_401k_loan: Option<Money>,
    pub home_value: Option<Money>,
    pub account_number: Option<String>,
    pub created_date: Option<String>,
    pub interest_paid_ytd: Option<Money>,
    pub provider_account_id: Option<i64>,
    pub collateral: Option<String>,
    pub dataset: Option<Vec<AccountDataset>>,
    pub running_balance: Option<Money>,
    pub source_id: Option<String>,
    pub due_date: Option<String>,
    pub frequency: Option<String>,
    pub maturity_amount: Option<Money>,
    pub associated_provider_account_id: Option<Vec<i64>>,
    pub is_asset: Option<bool>,
    pub principal_balance: Option<Money>,
    pub total_cash_limit: Option<Money>,
    pub maturity_date: Option<String>,
    pub minimum_amount_due: Option<Money>,
    pub annual_percentage_yield: Option<f64>,
    pub account_type: Option<String>,
    pub origination_date: Option<String>,
    pub total_vested_balance: Option<Money>,
    pub reward_balance: Option<Vec<RewardBalance>>,
    pub source_account_status: Option<String>,
    pub linked_account_ids: Option<Vec<i64>>,
    pub derived_apr: Option<f64>,
    pub policy_effective_date: Option<String>,
    pub total_unvested_balance: Option<Money>,
    pub annuity_balance: Option<Money>,
    pub account_name: Option<String>,
    pub total_credit_limit: Option<Money>,
    pub policy_status: Option<String>,
    pub short_balance: Option<Money>,
    pub lender: Option<String>,
    pub last_employee_contribution_amount: Option<Money>,
    pub provider_id: Option<String>,
    pub last_payment_date: Option<String>,
    pub primary_reward_unit: Option<String>,
    pub last_payment_amount: Option<Money>,
    pub remaining_balance: Option<Money>,
    pub user_classification: Option<String>,
    pub bank_transfer_code: Option<Vec<BankTransferCode>>,
    pub expiration_date: Option<String>,
    pub coverage: Option<Vec<Coverage>>,
    pub cash_apr: Option<f64>,
    pub auto_refresh: Option<AutoRefresh>,
    pub oauth_migration_status: Option<String>,
    pub displayed_name: Option<String>,
    pub full_account_number_list: Option<FullAccountNumberList>,
    pub amount_due: Option<Money>,
    pub current_level: Option<String>,
    pub original_loan_amount: Option<Money>,
    pub policy_to_date: Option<String>,
    pub loan_payoff_details: Option<LoanPayoffDetails>,
    pub payment_profile: Option<PaymentProfile>,
    #[serde(rename = "CONTAINER")]
    pub container: Option<String>,
    pub last_employee_contribution_date: Option<String>,
    pub last_payment: Option<Money>,
    pub recurring_payment: Option<Money>,
}
