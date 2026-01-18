# Sui Invariant Monitor - Workflow Diagrams

This document contains Mermaid diagrams illustrating the workflows and architecture of the Sui Invariant Monitor project.

## 1. Overall System Architecture

```mermaid
graph TB
    subgraph "User Interface"
        A[Web Browser]
    end
    
    subgraph "Frontend - Vercel"
        B[React App]
        C[Network Switcher]
        D[AI Analysis Form]
        E[Invariant Monitor]
    end
    
    subgraph "HTTPS Layer"
        F[Cloudflare Tunnel]
    end
    
    subgraph "Backend - VPS"
        G[Axum API Server]
        H[AI Analysis Engine]
        I[Invariant Engine]
        J[State Aggregator]
    end
    
    subgraph "External Services"
        K[OpenRouter API]
        L[Ollama Local]
        M[Sui RPC Mainnet]
        N[Sui RPC Testnet]
        O[Discord Webhook]
    end
    
    A --> B
    B --> C
    B --> D
    B --> E
    B -->|HTTPS| F
    F -->|HTTP| G
    G --> H
    G --> I
    G --> J
    H -->|AI Analysis| K
    H -->|AI Analysis| L
    J -->|Read State| M
    J -->|Read State| N
    I -->|Violations| O
```

## 2. AI Analysis Workflow

```mermaid
sequenceDiagram
    participant U as User
    participant F as Frontend
    participant B as Backend
    participant S as Sui RPC
    participant A as AI (OpenRouter/Ollama)
    
    U->>F: Enter Package ID
    U->>F: Select Network (mainnet/testnet)
    U->>F: Choose AI Model
    U->>F: Click "Analyze"
    
    F->>B: POST /api/analyze
    Note over F,B: {package_id, network, model, api_key}
    
    B->>S: GET /sui_getNormalizedMoveModule
    S-->>B: Module Metadata (structs, functions)
    
    B->>A: Analyze Move Code
    Note over B,A: Send module metadata + prompt
    A-->>B: Suggested Invariants
    Note over A,B: [{id, name, description, severity, formula}]
    
    B-->>F: Analysis Results
    F-->>U: Display Suggested Invariants
    
    U->>F: Click "Add to Monitoring"
    F->>B: POST /api/invariants/add
    B-->>F: Success
    F-->>U: Show in Monitor Grid
```

## 3. Real-time Monitoring Workflow

```mermaid
graph LR
    subgraph "Background Loop (10s interval)"
        A[Start] --> B[Fetch Monitored Objects]
        B --> C[Query Sui RPC]
        C --> D[Aggregate State]
        D --> E{Evaluate Invariants}
        E -->|OK| F[Update Status: OK]
        E -->|Violated| G[Update Status: Violated]
        G --> H[Send Discord Alert]
        F --> I[Wait 10s]
        H --> I
        I --> B
    end
    
    subgraph "Frontend Polling"
        J[Frontend] -->|Every 5s| K[GET /api/status]
        K --> L[Update UI]
        L --> J
    end
```

## 4. Network Switching Flow

```mermaid
stateDiagram-v2
    [*] --> Mainnet: Default
    
    Mainnet --> Testnet: User toggles switch
    Testnet --> Mainnet: User toggles switch
    
    state Mainnet {
        [*] --> MainnetRPC
        MainnetRPC --> AnalyzeMainnet: Analyze Package
        MainnetRPC --> MonitorMainnet: Monitor State
    }
    
    state Testnet {
        [*] --> TestnetRPC
        TestnetRPC --> AnalyzeTestnet: Analyze Package
        TestnetRPC --> MonitorTestnet: Monitor State
    }
    
    note right of Mainnet
        RPC: fullnode.mainnet.sui.io
    end note
    
    note right of Testnet
        RPC: fullnode.testnet.sui.io
    end note
```

## 5. Data Flow Architecture

```mermaid
flowchart TD
    subgraph "Input Layer"
        A[User Input]
        B[Package ID]
        C[Network Selection]
        D[AI Model Choice]
    end
    
    subgraph "Processing Layer"
        E[Metadata Fetcher]
        F[AI Analyzer]
        G[Invariant Generator]
    end
    
    subgraph "Storage Layer"
        H[In-Memory State]
        I[Monitored Invariants]
        J[Object IDs]
    end
    
    subgraph "Monitoring Layer"
        K[State Aggregator]
        L[Invariant Evaluator]
        M[Alert System]
    end
    
    subgraph "Output Layer"
        N[UI Updates]
        O[Discord Notifications]
        P[Status API]
    end
    
    A --> B
    A --> C
    A --> D
    
    B --> E
    C --> E
    E --> F
    D --> F
    F --> G
    
    G --> H
    G --> I
    B --> J
    
    J --> K
    I --> L
    K --> L
    L --> M
    
    H --> N
    L --> N
    M --> O
    L --> P
```

## 6. User Journey

```mermaid
journey
    title User Experience Flow
    section Discovery
      Visit Website: 5: User
      Read Documentation: 4: User
    section Analysis
      Enter Package ID: 5: User
      Select Network: 5: User
      Choose AI Model: 4: User
      Wait for Analysis: 3: User, Backend
      Review Suggestions: 5: User
    section Monitoring
      Add Invariants: 5: User
      View Status Grid: 5: User
      Receive Alerts: 4: User, System
      Remove Invariants: 5: User
    section Iteration
      Analyze Another Package: 5: User
      Switch Networks: 5: User
      Compare Results: 4: User
```

## 7. Component Interaction

```mermaid
graph TB
    subgraph "Frontend Components"
        A[Layout]
        B[NetworkSwitcher]
        C[AnalyzeContractForm]
        D[InvariantCard]
        E[StatusBadge]
    end
    
    subgraph "React Context"
        F[NetworkContext]
    end
    
    subgraph "API Client"
        G[client.ts]
    end
    
    subgraph "Backend Modules"
        H[api/handlers]
        I[analysis/llm]
        J[invariants/engine]
        K[sui_client]
        L[aggregator/state]
    end
    
    A --> B
    A --> C
    A --> D
    B --> F
    C --> F
    C --> G
    D --> E
    
    G --> H
    H --> I
    H --> J
    H --> K
    J --> L
    K --> L
```

## 8. Deployment Pipeline

```mermaid
graph LR
    A[Local Development] -->|git push| B[GitHub Repository]
    
    B -->|Auto Deploy| C[Vercel]
    C --> D[Frontend Production]
    
    B -->|git pull| E[VPS]
    E -->|cargo build| F[Backend Binary]
    F -->|Supervisor| G[Backend Running]
    
    G -->|Cloudflare Tunnel| H[HTTPS Endpoint]
    
    D -->|API Calls| H
    
    style D fill:#4da2ff
    style G fill:#4da2ff
    style H fill:#10b981
```

## 9. Error Handling Flow

```mermaid
flowchart TD
    A[User Action] --> B{Request Type}
    
    B -->|Analyze| C[Send to Backend]
    B -->|Monitor| D[Add to State]
    
    C --> E{Backend Response}
    E -->|Success| F[Display Results]
    E -->|Error| G{Error Type}
    
    G -->|Network Error| H[Show Retry Button]
    G -->|Invalid Package| I[Show Error Message]
    G -->|AI Timeout| J[Suggest Smaller Module]
    
    D --> K{Validation}
    K -->|Valid| L[Start Monitoring]
    K -->|Invalid| M[Show Validation Error]
    
    L --> N{Evaluation}
    N -->|Success| O[Update Status]
    N -->|Error| P[Mark as Error State]
    
    H --> A
    I --> A
    J --> A
    M --> A
```

## 10. Security & Access Control

```mermaid
graph TB
    subgraph "Public Access"
        A[Frontend HTTPS]
        B[Backend HTTPS]
    end
    
    subgraph "Authentication"
        C[No Auth Required]
        D[User Provides API Keys]
    end
    
    subgraph "Data Protection"
        E[HTTPS Encryption]
        F[CORS Policy]
        G[Input Validation]
    end
    
    subgraph "External Services"
        H[OpenRouter API Key]
        I[Discord Webhook]
        J[Sui RPC Public]
    end
    
    A --> E
    B --> E
    A --> F
    B --> G
    
    C --> D
    D --> H
    B --> I
    B --> J
    
    style E fill:#10b981
    style F fill:#10b981
    style G fill:#10b981
```

---

## Usage in README

You can embed these diagrams in your README.md by copying the mermaid code blocks:

\`\`\`mermaid
[paste diagram code here]
\`\`\`

## Notes

- All diagrams use Mermaid syntax compatible with GitHub and most markdown renderers
- Diagrams are designed to be clear and informative for technical documentation
- Color coding: Blue (#4da2ff) for Sui-related, Green (#10b981) for success/security

---

© 2026 Phú Nhuận Builder
