import { useEffect, useState } from 'react'
import './App.css'

function App() {
  const [message, setMessage] = useState('')
  useEffect(() => {
    fetch('http://localhost:4000/hello-world')
      .then(res => res.json())
      .then(data => setMessage(data.message))
      .catch(err => console.error(err))
  });

  return (
    <>
      <div>{message}</div>
    </>
  )
}

export default App
