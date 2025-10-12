type ErrorDisplayProps = {
  error: any;
};

export default function ErrorDisplay({ error }: ErrorDisplayProps) {
  if (!error) return null; 

  return (
    <p className="text-2xl text-yellow-600 flex items-center gap-2">
      <span>⚠️</span>
      {"An error occurred, please try again later or contact support on discord"}
    </p>
  );
}