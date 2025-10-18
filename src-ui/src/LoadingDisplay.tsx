import { useEffect, useState } from "react";
import './LoadingDisplay.css'

type LoadingDisplayProps = {
    display: { title_text: string, sub_text: string}
}

const LoadingDisplay = ({ display: {title_text, sub_text} }: LoadingDisplayProps) => {
  const [fadeKey, setFadeKey] = useState(0); // triggers animation on text change

  useEffect(() => {
    setFadeKey(prev => prev + 1);
  }, [sub_text])

  return (
    <div className="container">
      <div className="loading-wrapper">
          <div className="spinner"></div>
        <h1>{title_text}</h1>
        <p key={fadeKey} className="fade-text">{sub_text}</p>
      </div>
    </div>
  )
}

export default LoadingDisplay;