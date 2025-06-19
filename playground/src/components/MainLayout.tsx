import type React from "react";
import type { ScreenSizeType } from "@/types";
import { Tab, Tabs } from "./ui";

interface MainLayoutProps {
  screenSize: ScreenSizeType;
  optionsPanel: React.ReactNode;
  sourcePanel: React.ReactNode;
  formattedPanel: React.ReactNode;
  astPanel: React.ReactNode;
  irPanel: React.ReactNode;
}

export function MainLayout({
  screenSize,
  optionsPanel,
  sourcePanel,
  formattedPanel,
  astPanel,
  irPanel,
}: MainLayoutProps) {
  return (
    <div className="flex overflow-hidden min-h-0 h-full p-3 gap-2">
      {/* Wide Layout: 3 Columns */}
      {screenSize === "wide" && (
        <>
          <div className="panel flex-none w-[280px]">
            <div className="panel-header">Settings</div>
            <div className="panel-content">{optionsPanel}</div>
          </div>
          <div className="panel flex-1 min-w-0">
            <div className="panel-header">Source</div>
            <div className="panel-content">{sourcePanel}</div>
          </div>
          <div className="panel flex-1 min-w-0">
            <div className="panel-content">
              <Tabs defaultActiveTab="formatted">
                <Tab tid="formatted" label="Formatted">
                  {formattedPanel}
                </Tab>
                <Tab tid="ast" label="AST">
                  {astPanel}
                </Tab>
                <Tab tid="ir" label="Pretty IR">
                  {irPanel}
                </Tab>
              </Tabs>
            </div>
          </div>
        </>
      )}

      {/* Thin Layout: 1 Column (Full Width) */}
      {screenSize === "thin" && (
        <div className="panel">
          <div className="panel-content flex-1 min-w-0">
            <Tabs defaultActiveTab="source">
              <Tab tid="options" label="Settings">
                {optionsPanel}
              </Tab>
              <Tab tid="source" label="Source">
                {sourcePanel}
              </Tab>
              <Tab tid="formatted" label="Formatted">
                {formattedPanel}
              </Tab>
              <Tab tid="ast" label="AST">
                {astPanel}
              </Tab>
              <Tab tid="ir" label="Pretty IR">
                {irPanel}
              </Tab>
            </Tabs>
          </div>
        </div>
      )}
    </div>
  );
}
