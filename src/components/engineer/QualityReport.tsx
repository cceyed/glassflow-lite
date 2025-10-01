// T056: QualityReport component
import React from 'react';

interface QualityReportProps {
  report: {
    totalFiles: number;
    filesPassed: number;
    filesFailed: number;
    issuesFound: number;
    issuesFixed: number;
    criticalIssues: any[];
  };
  confidence: {
    overall: number;
    qualityScore: number;
    planAdherence: number;
    issuePenalty: number;
  };
}

export const QualityReport: React.FC<QualityReportProps> = ({ report, confidence }) => {
  const getConfidenceColor = (score: number): string => {
    if (score >= 85) return 'text-green-600';
    if (score >= 70) return 'text-yellow-600';
    return 'text-red-600';
  };

  const getConfidenceLabel = (score: number): string => {
    if (score >= 85) return 'High';
    if (score >= 70) return 'Medium';
    return 'Low';
  };

  return (
    <div className="quality-report">
      <h3 className="report-title">Quality Report</h3>

      <div className="confidence-section">
        <div className="confidence-overall">
          <span className="confidence-label">Overall Confidence</span>
          <span className={`confidence-value ${getConfidenceColor(confidence.overall)}`}>
            {confidence.overall.toFixed(1)}%
          </span>
          <span className="confidence-badge">{getConfidenceLabel(confidence.overall)}</span>
        </div>

        <div className="confidence-breakdown">
          <div className="confidence-item">
            <span>Quality Score</span>
            <span>{confidence.qualityScore.toFixed(1)}%</span>
          </div>
          <div className="confidence-item">
            <span>Plan Adherence</span>
            <span>{confidence.planAdherence.toFixed(1)}%</span>
          </div>
          <div className="confidence-item">
            <span>Issue Penalty</span>
            <span>{confidence.issuePenalty.toFixed(1)}%</span>
          </div>
        </div>
      </div>

      <div className="report-stats">
        <div className="stat-card">
          <div className="stat-value">{report.totalFiles}</div>
          <div className="stat-label">Total Files</div>
        </div>
        <div className="stat-card success">
          <div className="stat-value">{report.filesPassed}</div>
          <div className="stat-label">Passed</div>
        </div>
        <div className="stat-card error">
          <div className="stat-value">{report.filesFailed}</div>
          <div className="stat-label">Failed</div>
        </div>
      </div>

      <div className="issues-section">
        <div className="issues-summary">
          <span>{report.issuesFound} issues found</span>
          {report.issuesFixed > 0 && (
            <span className="issues-fixed">({report.issuesFixed} auto-fixed)</span>
          )}
        </div>

        {report.criticalIssues.length > 0 && (
          <div className="critical-issues">
            <h4>Critical Issues</h4>
            <ul className="issues-list">
              {report.criticalIssues.map((issue, idx) => (
                <li key={idx} className="issue-item">
                  <span className="issue-file">{issue.file}</span>
                  {issue.line && <span className="issue-line">:{issue.line}</span>}
                  <span className="issue-description">{issue.description}</span>
                </li>
              ))}
            </ul>
          </div>
        )}
      </div>
    </div>
  );
};
