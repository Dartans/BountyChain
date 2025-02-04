import { WalletProvider } from './contexts/WalletContext';
import { BountyList } from './components/BountyList';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

function App() {
  return (
    <WalletProvider>
      <div className="container mx-auto p-4">
        <header className="flex justify-between items-center mb-8">
          <h1 className="text-2xl font-bold">Bounty Board</h1>
          <WalletMultiButton />
        </header>
        
        <BountyList />
      </div>
    </WalletProvider>
  );
}

export default App;