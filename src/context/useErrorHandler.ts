import { useErrorContext } from "./ErrorContext";

export const useErrorHandler = () => {
  const { setError } = useErrorContext();

  const handleError = (error: unknown) => {
    if (error instanceof Error) {
      setError(error.message);
    } else if (typeof error === "string") {
      setError(error);
    } else if (
      error &&
      typeof (error as { message?: string }).message === "string"
    ) {
      setError((error as { message: string }).message);
    } else {
      setError("An unknown error occurred");
    }
  };

  return handleError;
};
