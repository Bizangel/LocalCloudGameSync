import { useEffect, useState } from "react";
import './LoadingDisplay.css'

type LoadingDisplayProps = {
    display: { title: string, subtext: string}
}

const LoadingDisplay = ({ display: {title, subtext} }: LoadingDisplayProps) => {
  const [fadeKey, setFadeKey] = useState(0); // triggers animation on text change

  useEffect(() => {
    setFadeKey(prev => prev + 1);
  }, [subtext])

  return (
    <div className="container">
      <div className="loading-wrapper">
          <div className="spinner"></div>
        <h1>{title}</h1>
        <p key={fadeKey} className="fade-text">{subtext}</p>
      </div>
    </div>
  )
}

export default LoadingDisplay;