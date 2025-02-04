// Type definitions for program accounts
export interface Bounty {
    publicKey: PublicKey;
    amount: number;
    githubIssue: string;
    expiresAt: number;
    status: BountyStatus;
    claimant?: PublicKey;
}

// Bounty status enum mirroring Rust implementation
export enum BountyStatus {
    Open = 0,
    Claimed = 1,
    Completed = 2,
    Expired = 3,
}

// Webhook payload type for GitHub events
export interface MergeWebhookPayload {
    action: 'closed';
    pull_request: {
        merged: boolean;
        body: string;
        title: string;
    };
}