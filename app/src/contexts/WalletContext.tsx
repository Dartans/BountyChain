import { createContext, useContext } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';

// Define wallet context structure
type WalletContextProps = {
  connected: boolean;
  publicKey?: PublicKey;
  connect: () => Promise<void>;
  disconnect: () => Promise<void>;
};

// Create context with default values
const WalletContext = createContext<WalletContextProps>({
  connected: false,
  connect: async () => {},
  disconnect: async () => {},
});

// Context provider component
export const WalletProvider = ({ children }: { children: React.ReactNode }) => {
  const { connected, publicKey, connect, disconnect } = useWallet();

  return (
    <WalletContext.Provider
      value={{
        connected,
        publicKey,
        connect: async () => {
          try {
            await connect();
          } catch (error) {
            console.error('Connection error:', error);
          }
        },
        disconnect: async () => {
          await disconnect();
        },
      }}
    >
      {children}
    </WalletContext.Provider>
  );
};

// Hook for easy context consumption
export const useWalletContext = () => useContext(WalletContext);