import React from 'react';
import { AutomationRule } from '../types';

interface AutomationRulesPageProps {
  automationRules: AutomationRule[];
}

const AutomationRulesPage: React.FC<AutomationRulesPageProps> = ({ automationRules }) => {
  return (
    <div className="automation">
      <p>Automation rules will be displayed here. This feature allows you to set up automatic email responses based on triggers and conditions.</p>
      
      <div className="rules-list">
        <h3>Your Automation Rules</h3>
        {automationRules.map(rule => (
          <div key={rule.id} className="rule-card">
            <h4>{rule.rule_name}</h4>
            <p><strong>Keywords:</strong> {rule.keywords.join(', ')}</p>
            <p><strong>Status:</strong> {rule.is_active ? 'Active' : 'Inactive'}</p>
            <p><strong>Created:</strong> {new Date(rule.created_at).toLocaleDateString()}</p>
          </div>
        ))}
      </div>
    </div>
  );
};

export default AutomationRulesPage;