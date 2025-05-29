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
    defaultActiveTab || tabs[0]?.id || ""
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
    <div className={`flex flex-col h-full overflow-hidden ${className}`}>
      {/* Tab Headers */}
      <div className="flex-shrink-0 flex">
        {" "}
        {tabs.map((tab, index) => (
          <button
            type="button"
            key={tab.id}
            onClick={() => handleTabChange(tab.id)}
            className={`
              flex-1 py-2 px-4 text-sm font-semibold transition-all duration-300
              cursor-pointer text-center relative border-b-2 bg-transparent
              ${
                activeTab === tab.id
                  ? "text-[var(--tab-active-text)] border-b-[var(--tab-active-border)] bg-[rgba(76,175,80,0.05)] shadow-[var(--shadow-medium)]"
                  : "text-[var(--tab-button-text)] border-b-[var(--tab-button-border)] hover:text-[var(--tab-active-text)] hover:bg-[rgba(76,175,80,0.08)] hover:shadow-[var(--shadow-soft)]"
              }
              ${index === 0 ? "rounded-tl-2xl" : ""}
              ${index === tabs.length - 1 ? "rounded-tr-2xl" : ""}
              ${tabClassName}
            `}
          >
            {tab.label}
          </button>
        ))}
      </div>

      {/* Tab Content */}
      <div className={`flex-1 overflow-hidden ${contentClassName}`}>
        {activeTabContent}
      </div>
    </div>
  );
}
