import { useState, useEffect } from "react";

import { greet } from "./tauri";

const App = () => {
  const [inputValue, setInputValue] = useState("");
  const [greeting, setGreeting] = useState("");

  useEffect(() => {
    greet().then(setGreeting);
  }, []);

  const handleGreet = async () => {
    const response = await greet(inputValue);
    setGreeting(response);
  };

  return (
    <>
      <div>{greeting}</div>
      <input
        type="text"
        value={inputValue}
        onChange={(e) => setInputValue(e.target.value)}
        placeholder="Enter your name"
      />
      <button onClick={handleGreet}>Greet</button>
    </>
  )
}

export default App
