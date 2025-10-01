import { useState } from 'react';

interface QuestionOption {
  value: string;
  label: string;
  description?: string;
}

interface Question {
  id: string;
  text: string;
  question_type: 'SingleChoice' | 'MultipleChoice' | 'YesNo' | 'FreeText' | 'ConfirmationWithDefault';
  options?: QuestionOption[];
  recommended_answer?: string;
  reasoning?: string;
  impact: 'High' | 'Medium' | 'Low';
}

interface QuestionDisplayProps {
  question: Question;
  questionNumber: number;
  totalQuestions: number;
  onAnswer: (answer: string) => void;
  onSkip?: () => void;
}

export function QuestionDisplay({
  question,
  questionNumber,
  totalQuestions,
  onAnswer,
  onSkip,
}: QuestionDisplayProps) {
  const [selectedAnswer, setSelectedAnswer] = useState<string>(
    question.recommended_answer || ''
  );
  const [multipleAnswers, setMultipleAnswers] = useState<Set<string>>(new Set());
  const [freeTextAnswer, setFreeTextAnswer] = useState<string>('');

  const handleSubmit = () => {
    let answer = '';
    
    switch (question.question_type) {
      case 'SingleChoice':
      case 'YesNo':
      case 'ConfirmationWithDefault':
        answer = selectedAnswer;
        break;
      case 'MultipleChoice':
        answer = Array.from(multipleAnswers).join(', ');
        break;
      case 'FreeText':
        answer = freeTextAnswer;
        break;
    }
    
    if (answer.trim()) {
      onAnswer(answer);
    }
  };

  const handleMultipleChoiceToggle = (value: string) => {
    const newAnswers = new Set(multipleAnswers);
    if (newAnswers.has(value)) {
      newAnswers.delete(value);
    } else {
      newAnswers.add(value);
    }
    setMultipleAnswers(newAnswers);
  };

  const impactColors = {
    High: 'border-red-500 bg-red-900/20',
    Medium: 'border-yellow-500 bg-yellow-900/20',
    Low: 'border-blue-500 bg-blue-900/20',
  };

  const impactTextColors = {
    High: 'text-red-400',
    Medium: 'text-yellow-400',
    Low: 'text-blue-400',
  };

  const isAnswerValid = () => {
    switch (question.question_type) {
      case 'SingleChoice':
      case 'YesNo':
      case 'ConfirmationWithDefault':
        return selectedAnswer.trim() !== '';
      case 'MultipleChoice':
        return multipleAnswers.size > 0;
      case 'FreeText':
        return freeTextAnswer.trim() !== '';
      default:
        return false;
    }
  };

  return (
    <div className={`p-6 rounded-lg border-2 ${impactColors[question.impact]}`}>
      {/* Header */}
      <div className="flex items-start justify-between mb-4">
        <div className="flex-1">
          <div className="flex items-center gap-3 mb-2">
            <span className="text-sm font-medium text-gray-400">
              Question {questionNumber} of {totalQuestions}
            </span>
            <span className={`text-xs font-semibold px-2 py-1 rounded ${impactTextColors[question.impact]} bg-gray-800`}>
              {question.impact} Impact
            </span>
          </div>
          <h3 className="text-lg font-semibold text-white">{question.text}</h3>
        </div>
      </div>

      {/* Reasoning */}
      {question.reasoning && (
        <div className="mb-4 p-3 bg-gray-800/50 rounded text-sm text-gray-300">
          <span className="font-medium text-gray-400">Why this matters:</span> {question.reasoning}
        </div>
      )}

      {/* Answer Input */}
      <div className="mb-4">
        {/* Single Choice */}
        {question.question_type === 'SingleChoice' && question.options && (
          <div className="space-y-2">
            {question.options.map((option) => (
              <label
                key={option.value}
                className={`flex items-start p-3 rounded-lg border cursor-pointer transition-colors ${
                  selectedAnswer === option.value
                    ? 'border-blue-500 bg-blue-900/20'
                    : 'border-gray-700 bg-gray-800/50 hover:border-gray-600'
                }`}
              >
                <input
                  type="radio"
                  name="answer"
                  value={option.value}
                  checked={selectedAnswer === option.value}
                  onChange={(e) => setSelectedAnswer(e.target.value)}
                  className="mt-1 mr-3"
                />
                <div className="flex-1">
                  <div className="font-medium text-white">{option.label}</div>
                  {option.description && (
                    <div className="text-sm text-gray-400 mt-1">{option.description}</div>
                  )}
                </div>
              </label>
            ))}
          </div>
        )}

        {/* Multiple Choice */}
        {question.question_type === 'MultipleChoice' && question.options && (
          <div className="space-y-2">
            <p className="text-sm text-gray-400 mb-2">Select all that apply:</p>
            {question.options.map((option) => (
              <label
                key={option.value}
                className={`flex items-start p-3 rounded-lg border cursor-pointer transition-colors ${
                  multipleAnswers.has(option.value)
                    ? 'border-blue-500 bg-blue-900/20'
                    : 'border-gray-700 bg-gray-800/50 hover:border-gray-600'
                }`}
              >
                <input
                  type="checkbox"
                  value={option.value}
                  checked={multipleAnswers.has(option.value)}
                  onChange={() => handleMultipleChoiceToggle(option.value)}
                  className="mt-1 mr-3"
                />
                <div className="flex-1">
                  <div className="font-medium text-white">{option.label}</div>
                  {option.description && (
                    <div className="text-sm text-gray-400 mt-1">{option.description}</div>
                  )}
                </div>
              </label>
            ))}
          </div>
        )}

        {/* Yes/No */}
        {question.question_type === 'YesNo' && (
          <div className="flex gap-3">
            <button
              onClick={() => setSelectedAnswer('Yes')}
              className={`flex-1 px-6 py-3 rounded-lg font-medium transition-colors ${
                selectedAnswer === 'Yes'
                  ? 'bg-green-600 text-white'
                  : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
              }`}
            >
              Yes
            </button>
            <button
              onClick={() => setSelectedAnswer('No')}
              className={`flex-1 px-6 py-3 rounded-lg font-medium transition-colors ${
                selectedAnswer === 'No'
                  ? 'bg-red-600 text-white'
                  : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
              }`}
            >
              No
            </button>
          </div>
        )}

        {/* Free Text */}
        {question.question_type === 'FreeText' && (
          <textarea
            value={freeTextAnswer}
            onChange={(e) => setFreeTextAnswer(e.target.value)}
            placeholder="Enter your answer..."
            className="w-full h-32 bg-gray-800 border border-gray-700 rounded-lg p-3 text-white placeholder-gray-500 focus:outline-none focus:border-blue-500 transition-colors resize-none"
          />
        )}

        {/* Confirmation with Default */}
        {question.question_type === 'ConfirmationWithDefault' && (
          <div className="space-y-3">
            <div className="p-3 bg-blue-900/20 border border-blue-500 rounded-lg">
              <p className="text-sm text-gray-300">
                <span className="font-medium text-blue-400">Recommended:</span>{' '}
                {question.recommended_answer}
              </p>
            </div>
            <div className="flex gap-3">
              <button
                onClick={() => setSelectedAnswer(question.recommended_answer || 'Yes')}
                className={`flex-1 px-6 py-3 rounded-lg font-medium transition-colors ${
                  selectedAnswer === question.recommended_answer
                    ? 'bg-blue-600 text-white'
                    : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
                }`}
              >
                Use Recommended
              </button>
              <button
                onClick={() => setSelectedAnswer('Custom')}
                className={`flex-1 px-6 py-3 rounded-lg font-medium transition-colors ${
                  selectedAnswer === 'Custom'
                    ? 'bg-gray-600 text-white'
                    : 'bg-gray-800 text-gray-300 hover:bg-gray-700'
                }`}
              >
                Choose Different
              </button>
            </div>
          </div>
        )}
      </div>

      {/* Actions */}
      <div className="flex gap-3">
        <button
          onClick={handleSubmit}
          disabled={!isAnswerValid()}
          className="flex-1 px-6 py-3 bg-blue-600 hover:bg-blue-700 disabled:bg-gray-700 disabled:cursor-not-allowed text-white font-medium rounded-lg transition-colors"
        >
          {questionNumber === totalQuestions ? 'Complete' : 'Next Question'}
        </button>
        {onSkip && (
          <button
            onClick={onSkip}
            className="px-6 py-3 bg-gray-700 hover:bg-gray-600 text-white font-medium rounded-lg transition-colors"
          >
            Skip
          </button>
        )}
      </div>
    </div>
  );
}
