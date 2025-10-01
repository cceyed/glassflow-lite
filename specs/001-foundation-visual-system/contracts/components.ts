/**
 * Component Contracts
 * Type definitions for all UI components in the design system
 */

import { ReactNode, CSSProperties } from "react";

// ============================================================================
// Base Component Props
// ============================================================================

export interface BaseComponentProps {
  className?: string;
  style?: CSSProperties;
  testId?: string;
}

export type ComponentState = "default" | "hover" | "active" | "focus" | "disabled";

// ============================================================================
// TitleBar Component
// ============================================================================

export interface TitleBarProps extends BaseComponentProps {
  projectName: string;
  onMinimize?: () => void;
  onMaximize?: () => void;
  onClose?: () => void;
}

export interface TitleBarDimensions {
  height: "40px";
}

// ============================================================================
// CommandInput Component
// ============================================================================

export interface CommandInputProps extends BaseComponentProps {
  value: string;
  onChange: (value: string) => void;
  onSubmit: (value: string) => void;
  placeholder?: string;
  autoFocus?: boolean;
}

export interface CommandInputDimensions {
  height: "60px";
  fontSize: "18px";
}

// ============================================================================
// AgentStatusBar Component
// ============================================================================

export interface AgentStatus {
  id: string;
  name: string;
  status: "idle" | "active" | "complete";
  progress: number;
  reasoning?: string[];
}

export interface AgentStatusBarProps extends BaseComponentProps {
  agents: AgentStatus[];
  onAgentClick?: (agentId: string) => void;
}

export interface AgentStatusBarDimensions {
  height: "48px";
  agentCount: 4;
}

// ============================================================================
// StatusBar Component
// ============================================================================

export interface StatusBarProps extends BaseComponentProps {
  projectPath: string;
  currentPhase: string;
  timeElapsed: string;
}

export interface StatusBarDimensions {
  height: "32px";
}

// ============================================================================
// Button Component
// ============================================================================

export type ButtonVariant = "primary" | "secondary" | "ghost";

export interface ButtonProps extends BaseComponentProps {
  variant?: ButtonVariant;
  disabled?: boolean;
  onClick?: () => void;
  children: ReactNode;
  type?: "button" | "submit" | "reset";
}

export interface ButtonStates {
  default: CSSProperties;
  hover: CSSProperties;
  active: CSSProperties;
  disabled: CSSProperties;
}

// ============================================================================
// Card Component
// ============================================================================

export interface CardProps extends BaseComponentProps {
  children: ReactNode;
  onClick?: () => void;
  hoverable?: boolean;
  active?: boolean;
}

export interface CardDimensions {
  borderRadius: "8px";
  padding: "24px";
}

// ============================================================================
// Modal Component
// ============================================================================

export interface ModalProps extends BaseComponentProps {
  isOpen: boolean;
  onClose: () => void;
  children: ReactNode;
  closeOnOverlayClick?: boolean;
  closeOnEscape?: boolean;
}

export interface ModalOverlay {
  background: "rgba(0, 0, 0, 0.9)";
  backdropBlur: "10px";
}

// ============================================================================
// Layout Region
// ============================================================================

export interface LayoutRegion {
  name: string;
  height: string;
  component: string;
  order: number;
}

export interface MainLayout {
  titleBar: LayoutRegion;
  commandInput: LayoutRegion;
  contentArea: LayoutRegion;
  agentStatusBar: LayoutRegion;
  statusBar: LayoutRegion;
}

// ============================================================================
// Component Dimensions
// ============================================================================

export interface ComponentDimensions {
  titleBar: TitleBarDimensions;
  commandInput: CommandInputDimensions;
  agentStatusBar: AgentStatusBarDimensions;
  statusBar: StatusBarDimensions;
  card: CardDimensions;
}

// ============================================================================
// Window Configuration
// ============================================================================

export interface WindowConfig {
  defaultWidth: 1400;
  defaultHeight: 900;
  minWidth: 1200;
  minHeight: 750;
}

// ============================================================================
// Component Exports
// ============================================================================

export interface DesignSystemComponents {
  TitleBar: React.FC<TitleBarProps>;
  CommandInput: React.FC<CommandInputProps>;
  AgentStatusBar: React.FC<AgentStatusBarProps>;
  StatusBar: React.FC<StatusBarProps>;
  Button: React.FC<ButtonProps>;
  Card: React.FC<CardProps>;
  Modal: React.FC<ModalProps>;
}
