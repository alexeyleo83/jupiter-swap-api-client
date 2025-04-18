use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};


use solana_sdk::pubkey::Pubkey;
use reqwest::Client;



const USDC_MINT: Pubkey = solana_sdk::pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
const USDT_MINT: Pubkey = solana_sdk::pubkey!("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");


async fn reqwest_async_client(r_client: reqwest::Client) -> reqwest::Response {
    r_client.get("http://127.0.0.1:18040/quote?inputMint=EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v&outputMint=Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB&amount=1000000").send().await.unwrap()
}

async fn quote_jup(jupiter_client: jupiter_swap_api_client_jup::JupiterSwapApiClient){
    
    let quote_req1 = jupiter_swap_api_client_jup::quote::QuoteRequest {
        amount: 1_000_000,
        input_mint: USDC_MINT,
        output_mint: USDT_MINT,
        // dexes: Some("Whirlpool,Meteora DLMM,Raydium CLMM".into()),
        slippage_bps: 500,
        max_accounts: Some(20),
        only_direct_routes: Some(true),
        ..jupiter_swap_api_client_jup::quote::QuoteRequest::default()
    };

    let quote_resp1 = jupiter_client
        .quote(&quote_req1)
        .await;
}
async fn quote_leo(jupiter_client: jupiter_swap_api_client_leo::JupiterSwapApiClient) {
    let quote_req1 = jupiter_swap_api_client_leo::quote::QuoteRequest {
        amount: 1_000_000,
        input_mint: USDC_MINT,
        output_mint: USDT_MINT,
        // dexes: Some("Whirlpool,Meteora DLMM,Raydium CLMM".into()),
        slippage_bps: 500,
        max_accounts: Some(20),
        only_direct_routes: Some(true),
        ..jupiter_swap_api_client_leo::quote::QuoteRequest::default()
    };

    let quote_resp1 = jupiter_client
        .quote(&quote_req1)
        .await;
}


fn criterion_benchmark_quote(c: &mut Criterion) {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let mut group = c.benchmark_group("quote_group");
    let reqw_client = reqwest::Client::new();
    let jupiter_client_leo = jupiter_swap_api_client_leo::JupiterSwapApiClient::new("http://127.0.0.1:18040".into());
    let jupiter_client_jup = jupiter_swap_api_client_jup::JupiterSwapApiClient::new("http://127.0.0.1:18040".into());

    group.bench_function("quote_leo", |b|
        b.to_async(&runtime).iter( || black_box(quote_leo(black_box(jupiter_client_leo.clone())))));

    group.bench_function("quote_jup", |b|
        b.to_async(&runtime).iter(  || black_box(quote_jup(black_box(jupiter_client_jup.clone())))));
    
    group.bench_function("reqwest_async_client", |b|
        b.to_async(&runtime).iter( || black_box(reqwest_async_client(black_box(reqw_client.clone())))));

    group.finish();
}

criterion_group!(jupiter, criterion_benchmark_quote);
criterion_main!(jupiter);