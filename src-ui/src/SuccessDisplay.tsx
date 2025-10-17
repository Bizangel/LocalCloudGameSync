import './SuccessDisplay.css'

type SuccessDisplayProps = {
  display: { title: string; subtext: string }
}

const SuccessDisplay = ({
  display: { title, subtext },
}: SuccessDisplayProps) => {
  return (
    <div className="container">
      <div className="success-wrapper">
        <div className="success-icon">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="64" height="64" fill="none">
            <circle cx="32" cy="32" r="30" stroke="#2ecc71" strokeWidth="4" />
            <path
              d="M18 33 L28 43 L46 22"
              stroke="#2ecc71"
              strokeWidth="4"
              strokeLinecap="round"
              strokeLinejoin="round"
            />
          </svg>
        </div>

        <h1>{title}</h1>
        <p>{subtext}</p>
      </div>
    </div>
  )
}

export default SuccessDisplay
