import './SuccessDisplay.css'

type SuccessDisplayProps = {
  display: { title_text: string; sub_text: string }
}

const SuccessDisplay = ({ display: { title_text, sub_text } }: SuccessDisplayProps) => {
  return (
    <div className="container">
      <div className="success-wrapper">
        <div className="success-icon">
          <div className="success-burst" />
          <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="0 0 64 64"
            width="64"
            height="64"
            fill="none"
          >
            <circle
              className="success-circle"
              cx="32"
              cy="32"
              r="30"
              stroke="#2ecc71"
              strokeWidth="4"
            />
            <path
              className="success-check"
              d="M18 33 L28 43 L46 22"
              stroke="#2ecc71"
              strokeWidth="4"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        </div>
        <h1>{title_text}</h1>
        <p>{sub_text}</p>
      </div>
    </div>
  )
}

export default SuccessDisplay
