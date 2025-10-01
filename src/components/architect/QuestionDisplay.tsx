import React from 'react';
import { GlassSurface } from '../design-system/GlassSurface';

interface QuestionOption {
  value: string;
  label: string;
  description?: string;
}

interface Question {
  id: string;
  text: string;
  questionType: string;
  options?: QuestionOption[];
  recommendedAnswer?: string;
  reasoning?: string;
}

interface QuestionDisplayProps {
  question: Question;
  progress: { current: number; total: number };
  onAnswer: (answer: string) => void;
}

export const QuestionDisplay: React.FC<QuestionDisplayProps> = ({
  question,
  progress,
  onAnswer,
}) => {
  return (
    <GlassSurface className="p-6 space-y-4">
      {/* Progress */}
      <div className="text-sm text-gray-400">
        Question {progress.current} of {progress.total}
      </div>
      
      {/* Question Text */}
      <h3 className="text-xl font-semibold text-white">{question.text}</h3>
      
      {/* Reasoning */}
      {question.reasoning && (
        <p className="text-sm text-gray-400 italic">{question.reasoning}</p>
      )}
      
      {/* Options */}
      {question.options && (
        <div className="space-y-2">
          {question.options.map((option) => (
            <button
              key={option.value}
              onClick={() => onAnswer(option.value)}
              className={`w-full text-left p-3 rounded-lg border transition-all duration-200 ${
                option.value === question.recommendedAnswer
                  ? 'border-blue-500 bg-blue-500/10 hover:bg-blue-500/20'
                  : 'border-gray-700 bg-gray-800/50 hover:bg-gray-700/50'
              }`}
            >
              <div className="font-medium text-white">{option.label}</div>
              {option.description && (
                <div className="text-sm text-gray-400 mt-1">{option.description}</div>
              )}
              {option.value === question.recommendedAnswer && (
                <div className="text-xs text-blue-400 mt-1">Recommended</div>
              )}
            </button>
          ))}
        </div>
      )}
      
      {/* Default Option */}
      <button
        onClick={() => onAnswer(question.recommendedAnswer || '')}
        className="w-full p-2 text-sm text-gray-400 hover:text-white transition-colors"
      >
        Use default (recommended) →
      </button>
    </GlassSurface>
  );
};
