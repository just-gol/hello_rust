use anyhow::Ok;

// 按万分比(bps)切分总额：floor + 余数归尾
const BPS: u128 = 10_000;

fn split_by_bps(total_smallest: u64, weights_bps: &[u16]) -> Result<Vec<u64>, &'static str> {
    // 1) 权重和校验（必须等于 10_000 bps）
    let sum: u32 = weights_bps.iter().map(|&w| w as u32).sum();
    if sum != 10_000 {
        return Err("bad weights sum");
    }

    // 2) 逐项 floor，最后一项吃余数，保证守恒
    let total = total_smallest as u128;
    let mut outs = vec![0u64; weights_bps.len()];
    let mut acc: u128 = 0;

    for (i, &w) in weights_bps.iter().enumerate() {
        let amt = if i + 1 == weights_bps.len() {
            total - acc // 余数全给最后一项
        } else {
            total * (w as u128) / BPS // floor
        };
        outs[i] = amt as u64;
        acc += amt;
    }
    Ok(outs)
}

fn cal(lamports: u64, weight: &[u16]) -> Result<Vec<u64>, &'static str> {
    let sum: u32 = weight.iter().map(|&x| x as u32).sum();
    if sum != 10_000 {
        return Err("bad weights sum");
    }
    let total = lamports as u128;

    let mut outs = vec![0u64; weight.len()];
    let mut acc: u128 = 0;
    for (i, &w) in weight.iter().enumerate() {
        let amt: u128 = if i + 1 == weight.len() {
            total - acc // 余数全给最后一项
        } else {
            total * (w as u128) / BPS // floor
        };
        outs[i] = amt as u64;
        acc += amt;
    }
    Ok(outs)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 示例1：总额 101（最小单位），权重 50% / 50%
    let out = split_by_bps(101, &[5000, 5000])?;
    println!("101 @ 50/50  -> {:?}  sum={}", out, out.iter().sum::<u64>());
    // 输出: [50, 51]  sum=101（守恒）

    // 示例2：总额 1_000_000，权重 60% / 40%
    let out = split_by_bps(1_000_000, &[6000, 4000])?;
    println!("1e6 @ 60/40  -> {:?}  sum={}", out, out.iter().sum::<u64>());
    // 输出: [600000, 400000]  sum=1000000（守恒）

    // ——用法——
    // 申购（用户把 total_smallest 打到合约）：
    //   let amounts = split_by_bps(total_smallest, weights)?;
    //   然后循环对每个成分按 amounts[i] 转账到金库（用户“拿到”的份额仍用 floor 计算）。
    //
    // 赎回（合约把底层打给用户）：
    //   let amounts = split_by_bps(total_smallest, weights)?;
    //   然后循环从金库给用户各自 ATA 转 amounts[i]（用户“拿到”的数量已是 floor）。
    Ok(())
}
