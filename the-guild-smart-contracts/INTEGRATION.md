# EAS Integration (TypeScript + wagmi)

This guide shows how to create on-chain attestations of TheGuild badges from a frontend using the EAS SDK, following the schema described in `README.md`.

- Schema: `bytes32 badgeName, bytes32 justification`
- Hardcoded schema id for now: set `SCHEMA_ID` to a placeholder and replace later

References:
- EAS SDK: [Creating on-chain attestations](https://docs.attest.org/docs/developer-tools/eas-sdk#creating-onchain-attestations)
- EAS SDK + wagmi: [wagmi integration](https://docs.attest.org/docs/developer-tools/sdk-wagmi)

## Install

```bash
npm install @ethereum-attestation-service/eas-sdk wagmi viem ethers @tanstack/react-query
```

## Choose network and addresses

```ts
// EAS contracts:
// Mainnet: 0xA1207F3BBa224E2c9c3c6D5aF63D0eb1582Ce587
// Sepolia: 0xC2679fBD37d54388Ce493F1DB75320D236e1815e
export const EAS_CONTRACT_ADDRESS = '0xC2679fBD37d54388Ce493F1DB75320D236e1815e'; // sepolia default for dev

// Replace with your real schema id once registered in EAS
export const SCHEMA_ID =
  '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef';
```

## Encoding helper for the schema

```ts
import { SchemaEncoder } from '@ethereum-attestation-service/eas-sdk';

export const schemaEncoder = new SchemaEncoder(
  'bytes32 badgeName, bytes32 justification'
);

export function encodeBadgeData(
  badgeNameBytes32: `0x${string}`,
  justificationBytes32: `0x${string}`
) {
  return schemaEncoder.encodeData([
    { name: 'badgeName', value: badgeNameBytes32, type: 'bytes32' },
    { name: 'justification', value: justificationBytes32, type: 'bytes32' },
  ]);
}
```

## Converting strings to bytes32

For UI inputs (badge name and justification), hash the string to `bytes32` for a fixed length value.

```ts
import { keccak256, encodePacked } from 'viem';

export function stringToBytes32(value: string): `0x${string}` {
  return keccak256(encodePacked(['string'], [value]));
}
```

## Hook: create a badge attestation (wagmi + ethers + EAS SDK)

This uses wagmi for connection state, ethers Signer for EAS SDK, and the EAS `attest` flow.

```ts
import { useAccount } from 'wagmi';
import { EAS } from '@ethereum-attestation-service/eas-sdk';
import { BrowserProvider } from 'ethers';
import { EAS_CONTRACT_ADDRESS, SCHEMA_ID } from './easConfig';
import { encodeBadgeData, stringToBytes32 } from './easEncoding';

export function useCreateBadgeAttestation() {
  const { isConnected } = useAccount();

  const createAttestation = async (
    recipient: `0x${string}`,
    badgeName: string,
    justification: string
  ) => {
    if (!isConnected) throw new Error('Wallet not connected');

    // 1) Get signer via ethers from injected provider (wagmi has already connected the wallet)
    const provider = new BrowserProvider((window as any).ethereum);
    const signer = await provider.getSigner();

    // 2) Init EAS with signer
    const eas = new EAS(EAS_CONTRACT_ADDRESS);
    eas.connect(signer);

    // 3) Encode data
    const encodedData = encodeBadgeData(
      stringToBytes32(badgeName),
      stringToBytes32(justification)
    );

    // 4) Attest
    const tx = await eas.attest({
      schema: SCHEMA_ID,
      data: {
        recipient,
        expirationTime: 0n, // no expiration
        revocable: true,
        refUID: '0x0000000000000000000000000000000000000000000000000000000000000000',
        data: encodedData,
        value: 0n,
      },
    });

    // 5) Wait for receipt and get the new attestation UID
    const receipt = await tx.wait();
    return receipt.uid;
  };

  return { createAttestation };
}
```

Notes:
- We use `ethers` Signer because EAS SDK expects an ethers-compatible signer/provider in its official examples.
- wagmi manages connection; the injected provider from the connected wallet is available via `window.ethereum`.

## Example component

```tsx
import { useState } from 'react';
import { useAccount, useConnect } from 'wagmi';
import { useCreateBadgeAttestation } from './useCreateBadgeAttestation';

export function BadgeAttestationForm() {
  const { isConnected } = useAccount();
  const { connect, connectors } = useConnect();
  const { createAttestation } = useCreateBadgeAttestation();

  const [recipient, setRecipient] = useState('' as `0x${string}`);
  const [badgeName, setBadgeName] = useState('');
  const [justification, setJustification] = useState('');
  const [txUid, setTxUid] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const onSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    if (!isConnected) return connect({ connector: connectors[0] });
    try {
      setLoading(true);
      const uid = await createAttestation(recipient, badgeName, justification);
      setTxUid(uid);
    } catch (err) {
      console.error(err);
      alert('Failed to create attestation');
    } finally {
      setLoading(false);
    }
  };

  return (
    <form onSubmit={onSubmit}>
      <input
        placeholder="Recipient 0x..."
        value={recipient}
        onChange={(e) => setRecipient(e.target.value as `0x${string}`)}
      />
      <input
        placeholder="Badge name"
        value={badgeName}
        onChange={(e) => setBadgeName(e.target.value)}
      />
      <input
        placeholder="Justification"
        value={justification}
        onChange={(e) => setJustification(e.target.value)}
      />
      <button type="submit" disabled={loading}>
        {loading ? 'Submitting…' : 'Create Attestation'}
      </button>
      {txUid && <p>Created attestation UID: {txUid}</p>}
    </form>
  );
}
```

## Reading an attestation

```ts
import { EAS } from '@ethereum-attestation-service/eas-sdk';
import { BrowserProvider } from 'ethers';
import { EAS_CONTRACT_ADDRESS } from './easConfig';

export async function getAttestation(uid: `0x${string}`) {
  const provider = new BrowserProvider((window as any).ethereum);
  const eas = new EAS(EAS_CONTRACT_ADDRESS);
  eas.connect(await provider.getSigner());
  return eas.getAttestation(uid);
}
```

## Tips

- Replace `SCHEMA_ID` with the actual schema id you register in EAS.
- Keep the schema definition exactly as specified: `bytes32 badgeName, bytes32 justification`.
- If you prefer using viem’s clients directly, you can still obtain an `ethers` Signer (as shown) solely for interacting with EAS SDK per its docs.

Links: [EAS SDK attestations](https://docs.attest.org/docs/developer-tools/eas-sdk#creating-onchain-attestations), [EAS + wagmi](https://docs.attest.org/docs/developer-tools/sdk-wagmi)


