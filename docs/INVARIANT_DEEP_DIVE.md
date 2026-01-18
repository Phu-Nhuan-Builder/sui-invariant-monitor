# How Invariants Work - Deep Dive

This document explains in detail how invariants are checked, how errors are reported, and what real-world problems this project solves.

## 🎯 Real-World Problem

### The Problem: Smart Contract Bugs Cost Billions

In DeFi protocols, smart contracts manage billions of dollars. A single bug can lead to:

**Real Examples:**
- **Poly Network Hack (2021)**: $600M stolen due to access control bug
- **Wormhole Bridge (2022)**: $325M exploited due to signature verification bug
- **Euler Finance (2023)**: $197M drained due to donation attack
- **Cream Finance (2021)**: $130M lost due to reentrancy bug

### The Core Issue: Invariant Violations

All these hacks happened because **protocol invariants were violated**:

1. **Total Supply Conservation**: `total_supply = sum(all_balances)`
   - If violated → tokens created from thin air
   
2. **Collateralization**: `total_borrowed <= total_collateral * collateral_ratio`
   - If violated → undercollateralized loans, protocol insolvency

3. **Accounting Balance**: `reserves + borrowed = supply`
   - If violated → accounting mismatch, lost funds

4. **Access Control**: `only_admin_can_mint`
   - If violated → unauthorized minting

### Current Solutions Are Inadequate

**Manual Audits:**
- ❌ Expensive ($50k-$200k per audit)
- ❌ One-time only (code changes after audit)
- ❌ Human error possible
- ❌ Can't catch runtime violations

**Static Analysis:**
- ❌ Can't detect runtime state issues
- ❌ Limited to code-level bugs
- ❌ No real-time monitoring

**Our Solution: AI + Real-time Monitoring**
- ✅ AI automatically identifies invariants from code
- ✅ Continuous monitoring (every 10 seconds)
- ✅ Immediate alerts when violations occur
- ✅ Cost-effective (no per-audit fees)

---

## 🔍 How Invariants Are Checked

### Step 1: AI Identifies Invariants

When you analyze a package, the AI:

1. **Reads Move Code** from Sui blockchain
2. **Analyzes Structs & Functions**:
   ```move
   struct Coin<phantom T> has key, store {
       id: UID,
       balance: Balance<T>
   }
   
   public fun mint<T>(cap: &mut TreasuryCap<T>, value: u64): Coin<T>
   public fun burn<T>(cap: &mut TreasuryCap<T>, coin: Coin<T>)
   ```

3. **Suggests Invariants**:
   - "Total supply must equal sum of all coin balances"
   - "Only TreasuryCap holder can mint"
   - "Burning decreases total supply"

### Step 2: User Adds to Monitoring

User clicks "Add to Monitoring" → Backend stores:
```rust
struct MonitoredInvariant {
    id: String,              // "total_supply_conservation"
    name: String,            // "Total Supply Conservation"
    description: String,     // "Ensures no tokens created/destroyed"
    severity: Severity,      // Critical, High, Medium, Low
    formula: String,         // "total_supply == sum(balances)"
}
```

### Step 3: Background Monitoring Loop

Every 10 seconds, the backend:

```rust
loop {
    // 1. Fetch current state from Sui blockchain
    let state = fetch_protocol_state().await;
    
    // 2. Evaluate all monitored invariants
    for invariant in monitored_invariants {
        let result = invariant.evaluate(&state);
        
        match result.status {
            InvariantStatus::Ok => {
                // ✅ Invariant satisfied
                log::info!("Invariant {} OK", invariant.id);
            }
            InvariantStatus::Violated => {
                // ❌ Invariant violated!
                log::error!("VIOLATION: {}", result.message);
                send_discord_alert(&result).await;
            }
            InvariantStatus::Error => {
                // ⚠️ Evaluation error
                log::warn!("Error evaluating {}", invariant.id);
            }
        }
    }
    
    // 3. Wait 10 seconds
    tokio::time::sleep(Duration::from_secs(10)).await;
}
```

---

## 📊 Invariant Evaluation Process

### Example: Total Supply Conservation

**Invariant**: `total_supply == total_reserves + total_borrowed`

**Code Implementation**:
```rust
fn evaluate(&self, state: &ProtocolState) -> InvariantResult {
    // 1. Calculate expected value
    let expected = state.total_reserves + state.total_borrowed;
    let actual = state.total_supply;
    
    // 2. Create computation record (for debugging)
    let computation = InvariantComputation::new("total_supply == total_reserves + total_borrowed")
        .with_input("total_supply", actual)
        .with_input("total_reserves", state.total_reserves)
        .with_input("total_borrowed", state.total_borrowed)
        .with_input("expected", expected)
        .with_result(format!("{} {} {}", actual, 
            if actual == expected { "==" } else { "!=" }, 
            expected));
    
    // 3. Check invariant
    if actual == expected {
        // ✅ OK
        InvariantResult::ok(
            self.id(),
            self.name(),
            self.description(),
            computation
        )
    } else {
        // ❌ VIOLATED
        InvariantResult::violated(
            self.id(),
            self.name(),
            self.description(),
            computation,
            &format!("Supply mismatch: actual {} != expected {}", actual, expected)
        )
    }
}
```

### What Gets Checked

For each invariant, we check:

1. **Current State** (from Sui RPC):
   ```json
   {
     "total_supply": 1000000,
     "total_reserves": 800000,
     "total_borrowed": 200000,
     "total_collateral": 250000
   }
   ```

2. **Invariant Formula**:
   - `total_supply == total_reserves + total_borrowed`
   - `1000000 == 800000 + 200000`
   - `1000000 == 1000000` ✅

3. **Result**:
   - If match → Status: OK
   - If mismatch → Status: VIOLATED

---

## 🚨 How Errors Are Reported

### 1. Frontend UI Updates

Every 5 seconds, frontend polls:
```typescript
const { data: status } = useQuery({
  queryKey: ['status'],
  queryFn: fetchStatus,
  refetchInterval: 5000  // Poll every 5s
});
```

**UI Shows**:
```
┌─────────────────────────────────────┐
│ Total Supply Conservation           │
│ Status: ❌ VIOLATED                 │
│ Last Check: 2026-01-18 16:40:30    │
│                                     │
│ Computation:                        │
│ total_supply = 1000000              │
│ total_reserves = 800000             │
│ total_borrowed = 150000             │
│ expected = 950000                   │
│ Result: 1000000 != 950000           │
│                                     │
│ Message: Supply mismatch            │
└─────────────────────────────────────┘
```

### 2. Discord Webhook Alert

When violation detected:
```rust
async fn send_discord_alert(result: &InvariantResult) {
    let webhook_url = env::var("DISCORD_WEBHOOK_URL")?;
    
    let message = json!({
        "embeds": [{
            "title": "⚠️ Invariant Violation Detected",
            "description": format!("**{}**\n{}", result.name, result.description),
            "color": 15158332,  // Red
            "fields": [
                {
                    "name": "Invariant ID",
                    "value": result.id,
                    "inline": true
                },
                {
                    "name": "Severity",
                    "value": "CRITICAL",
                    "inline": true
                },
                {
                    "name": "Violation Message",
                    "value": result.message,
                    "inline": false
                },
                {
                    "name": "Computation",
                    "value": format_computation(&result.computation),
                    "inline": false
                }
            ],
            "timestamp": Utc::now().to_rfc3339()
        }]
    });
    
    reqwest::Client::new()
        .post(&webhook_url)
        .json(&message)
        .send()
        .await?;
}
```

**Discord Message**:
```
🚨 Invariant Violation Detected

Total Supply Conservation
Ensures total supply equals reserves + borrowed

Invariant ID: total_supply_conservation
Severity: CRITICAL

Violation Message:
Supply mismatch: actual 1000000 != expected 950000

Computation:
total_supply = 1000000
total_reserves = 800000
total_borrowed = 150000
expected = 950000
Result: 1000000 != 950000

Timestamp: 2026-01-18T16:40:30Z
```

### 3. Logs (for debugging)

```
2026-01-18T16:40:30Z ERROR sui_invariant_monitor: VIOLATION DETECTED
  invariant_id: total_supply_conservation
  invariant_name: Total Supply Conservation
  message: Supply mismatch: actual 1000000 != expected 950000
  computation:
    total_supply: 1000000
    total_reserves: 800000
    total_borrowed: 150000
    expected: 950000
    result: 1000000 != 950000
```

---

## 🔄 Complete Flow Example

### Scenario: Detecting an Exploit

1. **Normal State** (10:00:00):
   ```
   total_supply: 1,000,000
   total_reserves: 800,000
   total_borrowed: 200,000
   ✅ Invariant OK: 1,000,000 == 800,000 + 200,000
   ```

2. **Exploit Happens** (10:00:05):
   - Attacker exploits bug to mint 50,000 tokens
   - New state:
     ```
     total_supply: 1,050,000  (increased!)
     total_reserves: 800,000  (unchanged)
     total_borrowed: 200,000  (unchanged)
     ```

3. **Next Check** (10:00:10):
   ```
   ❌ VIOLATION: 1,050,000 != 800,000 + 200,000
   ```

4. **Alert Sent** (10:00:10):
   - Discord notification sent to team
   - UI shows red badge
   - Logs record violation

5. **Team Response** (10:00:15):
   - Team sees Discord alert
   - Checks UI for details
   - Pauses protocol
   - Investigates exploit
   - Fixes bug

**Time to Detection**: 5-10 seconds (vs hours/days with manual monitoring)

---

## 💡 Why This Matters

### Traditional Approach
```
Deploy → Wait → Hack Happens → Users Report → Team Investigates → Too Late
Time: Hours to Days
Result: Funds Lost
```

### Our Approach
```
Deploy → Monitor → Violation Detected → Alert Sent → Team Responds → Protocol Paused
Time: 5-10 Seconds
Result: Funds Protected
```

### Key Benefits

1. **Early Detection**:
   - Violations detected in seconds, not hours
   - Team can respond before major damage

2. **Continuous Monitoring**:
   - 24/7 automated checking
   - No human needed to watch

3. **Detailed Diagnostics**:
   - Exact values shown
   - Computation trace for debugging
   - Timestamp for correlation

4. **Cost-Effective**:
   - No per-audit fees
   - Runs on cheap VPS (~$10/month)
   - Scales to monitor multiple protocols

---

## 🎯 Real-World Impact

If this tool existed during past hacks:

**Poly Network ($600M)**:
- Invariant: "Only authorized addresses can call critical functions"
- Would detect: Unauthorized function call
- Time saved: Hack took hours, detection would be seconds

**Wormhole ($325M)**:
- Invariant: "Signature verification must pass"
- Would detect: Invalid signature accepted
- Time saved: Immediate detection vs 2 hours

**Euler Finance ($197M)**:
- Invariant: "Collateral must exceed borrowed amount"
- Would detect: Undercollateralized position
- Time saved: Real-time vs overnight

---

## 📝 Summary

**How Invariants Are Checked:**
1. AI analyzes code → suggests invariants
2. User adds to monitoring
3. Backend fetches state every 10s
4. Evaluates each invariant
5. Reports violations immediately

**How Errors Are Reported:**
1. UI shows red badge + details
2. Discord webhook sends alert
3. Logs record for debugging

**Real Problem Solved:**
- Prevents multi-million dollar hacks
- Detects violations in seconds
- Protects user funds
- Increases protocol trust

**This is not just a monitoring tool - it's a security system that could save the DeFi ecosystem billions of dollars.**

---

© 2026 Phú Nhuận Builder
