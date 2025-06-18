import {
  Children,
  isValidElement,
  type ReactNode,
  useEffect,
  useState,
} from "react";

export interface TabItem {
  key: string;
  label: string;
  content: ReactNode;
}

export interface TabProps {
  tid: string;
  label: string;
  children: ReactNode;
}

export interface TabsProps {
  children?: ReactNode;
  activeTab?: string;
  onTabChange?: (tabId: string) => void;
  defaultActiveTab?: string;
  className?: string;
  tabClassName?: string;
  contentClassName?: string;
}

// Tab component - used for declarative JSX syntax
export function Tab({ children }: TabProps) {
  // This component is just a container, the actual rendering is handled by Tabs
  return <>{children}</>;
}

export function Tabs({
  children,
  activeTab: externalActiveTab,
  onTabChange: externalOnTabChange,
  defaultActiveTab,
  className = "",
  tabClassName = "",
  contentClassName = "",
}: TabsProps) {
  // Extract tabs from children using declarative JSX syntax
  const tabs: TabItem[] = children
    ? (Children.map(children, (child) => {
        if (isValidElement(child) && child.type === Tab) {
          const tabProps = child.props as TabProps;
          return {
            key: tabProps.tid,
            label: tabProps.label,
            content: tabProps.children,
          };
        }
        return null;
      })?.filter(Boolean) as TabItem[])
    : [];

  // Internal state management
  const [internalActiveTab, setInternalActiveTab] = useState<string>(
    defaultActiveTab || tabs[0]?.key || ""
  );

  // Determine if we're using external or internal state management
  const isControlled = externalActiveTab !== undefined;
  const activeTab = isControlled ? externalActiveTab : internalActiveTab;

  // Handle tab changes
  const handleTabChange = (tabId: string) => {
    if (isControlled) {
      // External state management
      externalOnTabChange?.(tabId);
    } else {
      // Internal state management
      setInternalActiveTab(tabId);
    }
  };

  // Sync internal state with external prop changes (for controlled mode)
  useEffect(() => {
    if (isControlled && externalActiveTab) {
      setInternalActiveTab(externalActiveTab);
    }
  }, [externalActiveTab, isControlled]);

  const activeTabContent = tabs.find((tab) => tab.key === activeTab)?.content;

  return (
    <div className={`flex flex-col h-full min-h-0 ${className}`}>
      <div className="tabs tabs-border flex-shrink-0">
        {tabs.map((tab) => {
          const isActive = activeTab === tab.key;
          const buttonClasses = ["tab", isActive && "active", tabClassName]
            .filter(Boolean)
            .join(" ");

          return (
            <button
              key={tab.key}
              role="tab"
              type="button"
              className={buttonClasses}
              aria-selected={isActive}
              onClick={() => handleTabChange(tab.key)}
            >
              {tab.label}
            </button>
          );
        })}
      </div>
      {/* Tab Content - This must be flex-1 and have min-h-0 for proper shrinking */}
      <div className={`flex-1 min-h-0 overflow-hidden ${contentClassName}`}>
        {activeTabContent}
      </div>
    </div>
  );
}
