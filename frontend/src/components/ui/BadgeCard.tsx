import React, { useRef } from "react";
import { cn } from "@/lib/utils";
import { Card } from "@/components/ui/card";

interface BadgeCardProps {
  children: React.ReactNode;
  className?: string;
  foregroundIcon?: React.ReactNode; // NEW optional prop
}

export function BadgeCard({
  children,
  className,
  foregroundIcon,
}: BadgeCardProps) {
  const tiltRef = useRef<HTMLDivElement>(null);
  const contentRef = useRef<HTMLDivElement>(null);
  const iconRef = useRef<HTMLDivElement>(null);

  const onEnter = () => {
    if (!tiltRef.current || !contentRef.current) return;
    tiltRef.current.style.transition = "transform 200ms ease-out";
    contentRef.current.style.transition = "transform 200ms ease-out";
    if (iconRef.current)
      iconRef.current.style.transition = "transform 200ms ease-out";

    tiltRef.current.style.transform = "rotateX(0deg) rotateY(0deg)";
    contentRef.current.style.transform = "translateZ(30px)";
    if (iconRef.current) iconRef.current.style.transform = "translateZ(180px)";
  };

  const onMove = (e: React.MouseEvent<HTMLDivElement>) => {
    if (!tiltRef.current || !contentRef.current) return;
    const rect = tiltRef.current.getBoundingClientRect();
    const x = (e.clientX - rect.left) / rect.width - 0.5;
    const y = (e.clientY - rect.top) / rect.height - 0.5;
    const rotateX = y * -12;
    const rotateY = x * 12;

    tiltRef.current.style.transition = "transform 120ms ease-out";
    contentRef.current.style.transition = "transform 120ms ease-out";
    if (iconRef.current)
      iconRef.current.style.transition = "transform 120ms ease-out";

    tiltRef.current.style.transform = `rotateX(${rotateX}deg) rotateY(${rotateY}deg)`;
    contentRef.current.style.transform = `translateZ(30px) translate(${x * 8}px, ${y * 8}px)`;
    if (iconRef.current) {
      iconRef.current.style.transform = `translateZ(60px) translate(${x * 12}px, ${y * 12}px)`;
    }
  };

  const onLeave = () => {
    if (!tiltRef.current || !contentRef.current) return;
    tiltRef.current.style.transition = "transform 350ms ease-out";
    contentRef.current.style.transition = "transform 350ms ease-out";
    if (iconRef.current)
      iconRef.current.style.transition = "transform 350ms ease-out";

    tiltRef.current.style.transform = "rotateX(0deg) rotateY(0deg)";
    contentRef.current.style.transform = "translateZ(0px)";
    if (iconRef.current) iconRef.current.style.transform = "translateZ(0px)";
  };

  return (
    <div
      style={{ perspective: "1000px" }}
      onMouseEnter={onEnter}
      onMouseMove={onMove}
      onMouseLeave={onLeave}
    >
      <div
        ref={tiltRef}
        className="will-change-transform"
        style={{
          transformStyle: "preserve-3d",
          transform: "rotateX(0deg) rotateY(0deg)",
        }}
      >
        <Card
          className={cn(
            "relative bg-white/70 dark:bg-gray-800/70 backdrop-blur-md transition-shadow duration-300 hover:shadow-lg",
            className
          )}
        >
          {" "}
          {foregroundIcon && (
            <div
              ref={iconRef}
              className="flex items-center justify-center pointer-events-none will-change-transform mb-4"
              style={{ transform: "translateZ(0px)" }}
            >
              {foregroundIcon}
            </div>
          )}
          <div
            ref={contentRef}
            className="will-change-transform"
            style={{ transform: "translateZ(0px)" }}
          >
            {children}
          </div>
        </Card>
      </div>
    </div>
  );
}
