canister id写给哪个 canister充cycle
@param canister_id : to which canister
let canister_id = Principal::from_text("").expect("wrap principal failed");

crt path 写 plug 导出的密钥的绝对路径
@param crt path : the .crt file exported by plug wallet 
let crt_path = String::from("");

amount : 冲多少cycle，如果 1T 就写 1e12
@param amount : cycle amount, 1 T = 1e12 
let amount = 1e12 as u64;
