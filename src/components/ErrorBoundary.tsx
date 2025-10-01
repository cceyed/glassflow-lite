import { Component, ErrorInfo, ReactNode } from 'react';
import { Card } from '../design-system/components/Card';
import { Button } from '../design-system/components/Button';

interface Props {
  children: ReactNode;
  fallback?: ReactNode;
}

interface State {
  hasError: boolean;
  error: Error | null;
  errorInfo: ErrorInfo | null;
}

export class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null,
    errorInfo: null,
  };

  public static getDerivedStateFromError(error: Error): State {
    return {
      hasError: true,
      error,
      errorInfo: null,
    };
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('ErrorBoundary caught an error:', error, errorInfo);
    this.setState({
      error,
      errorInfo,
    });
  }

  private handleReset = () => {
    this.setState({
      hasError: false,
      error: null,
      errorInfo: null,
    });
  };

  public render() {
    if (this.state.hasError) {
      if (this.props.fallback) {
        return this.props.fallback;
      }

      return (
        <div className="min-h-screen bg-background text-text-primary flex items-center justify-center p-8">
          <Card className="max-w-2xl w-full">
            <div className="text-center">
              <div className="text-6xl mb-6">⚠️</div>
              <h1 className="text-2xl font-bold mb-4">Something went wrong</h1>
              <p className="text-text-secondary mb-6">
                An unexpected error occurred in the application.
              </p>
              
              {this.state.error && (
                <Card className="bg-black/50 mb-6 text-left">
                  <div className="mb-4">
                    <h3 className="text-sm font-semibold text-text-secondary mb-2">Error Message:</h3>
                    <pre className="text-xs font-mono text-red-400 overflow-x-auto">
                      {this.state.error.message}
                    </pre>
                  </div>
                  
                  {this.state.errorInfo && (
                    <div>
                      <h3 className="text-sm font-semibold text-text-secondary mb-2">Stack Trace:</h3>
                      <pre className="text-xs font-mono text-text-tertiary overflow-x-auto max-h-48 overflow-y-auto">
                        {this.state.errorInfo.componentStack}
                      </pre>
                    </div>
                  )}
                </Card>
              )}
              
              <div className="flex gap-4 justify-center">
                <Button onClick={this.handleReset}>
                  🔄 Try Again
                </Button>
                <Button
                  variant="secondary"
                  onClick={() => window.location.reload()}
                >
                  ↻ Reload Page
                </Button>
              </div>
            </div>
          </Card>
        </div>
      );
    }

    return this.props.children;
  }
}
