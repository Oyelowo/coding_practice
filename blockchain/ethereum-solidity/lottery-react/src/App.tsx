
import { useEffect, useState } from 'react';
import './App.css';
import useForm from './hooks/useForm';
import lottery from './lottery';
import web3 from './web3';



function App() {
  const [manager, setManager] = useState()
  const [players, setPlayers] = useState<string[]>()
  const [balance, setBalance] = useState<string>()
  const [message, setMessage] = useState('')

  const { getInputAttributes } = useForm({ etherValue: '' })
  const etherInput = getInputAttributes('etherValue')


  useEffect(() => {
    const getManager = async () => {
      const manager = await lottery.methods.manager().call();
      const players = await lottery.methods.getPlayers().call();
      const balance = await web3.eth.getBalance(lottery.options.address);
      setManager(manager);
      setPlayers(players);
      setBalance(balance);
    }
    getManager();
  })


  const handleSubmit = async (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    try {
      const accounts = await web3.eth.getAccounts();
      setMessage('Waiting for transaction success...')
      await lottery.methods.enter().send({
        from: accounts?.[0],
        value: web3.utils.toWei(etherInput.value, 'ether')
      });
      setMessage('Hooorray! You have been entered! ...')
    } catch (error) {
      console.log("Something went wrong", error);
    }

  }

  const handleClick = async () => {
    const accounts = await web3.eth.getAccounts();
    setMessage('Waiting for transaction success...');

    await lottery.methods.pickWinner().send({ from: accounts[0], });
    setMessage('Congrats! A winner has been picked')
  }

  return (
    <div className="App">
      <h1>Lottery Contract</h1>
      <div>
        Contract is managed by {manager}. <br />
        There are currently {players?.length} people competing to win {web3.utils.fromWei(balance ?? '', 'ether')} ether;
      </div>
      <hr />

      <form onSubmit={handleSubmit}>
        <h4>Want to try the lottery out?</h4>

        <label htmlFor={etherInput.name}>Amount of ether to enter:</label>
        <input {...etherInput} />


        <button>Enter</button>
      </form>

      <hr />
      <h4>Ready to pick a winner?</h4>

      <button onClick={handleClick}>Pick a winner</button>
      <hr />
      <h1>{message}</h1>
    </div>
  );
}

export default App;
