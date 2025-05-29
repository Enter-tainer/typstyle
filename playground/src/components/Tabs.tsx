import {
  Children,
  type ReactNode,
  isValidElement,
  useEffect,
  useState,
} from "react";

export interface TabItem {
  id: string;
  label: string;
  content: ReactNode;
}

export interface TabProps {
  id: string;
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
            id: tabProps.id,
            label: tabProps.label,
            content: tabProps.children,
          };
        }
        return null;
      })?.filter(Boolean) as TabItem[])
    : [];

  // Internal state management
  const [internalActiveTab, setInternalActiveTab] = useState<string>(
    defaultActiveTab || tabs[0]?.id || "",
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

  const activeTabContent = tabs.find((tab) => tab.id === activeTab)?.content;

  return (
    <div className={`tabs-container ${className}`}>
      {/* Tab Headers */}
      <div className="tabs-header-list">
        {tabs.map((tab, index) => {
          const buttonStateClasses = [];
          if (activeTab === tab.id) {
            buttonStateClasses.push("active");
          }

          const buttonClasses = `tab-button ${buttonStateClasses.join(
            " ",
          )} ${tabClassName}`;

          return (
            <button
              key={tab.id}
              type="button"
              className={buttonClasses.trim()}
              onClick={() => handleTabChange(tab.id)}
              aria-selected={activeTab === tab.id}
              role="tab"
            >
              {tab.label}
            </button>
          );
        })}
      </div>

      {/* Tab Content */}
      <div className={`tabs-content ${contentClassName}`}>
        {activeTabContent}
      </div>
    </div>
  );
}
