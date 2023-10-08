import React from "react";

interface IconProps {
  fill?: string;
  filled?: boolean;
  size?: number;
  height?: number;
  width?: number;
  label?: string;

  [key: string]: any;
}

export const StopIcon = ({
                           fill = 'currentColor',
                           filled,
                           size,
                           height,
                           width,
                           label,
                           ...props
                         }: IconProps) => {
  return (
      <svg
          width={size || width || 32}
          height={size || height || 32}
          viewBox="0 0 32 32"
          fill={filled ? fill : 'none'}
          xmlns="http://www.w3.org/2000/svg"
          {...props}
      >
        <path d="M16,0C7.164,0,0,7.164,0,16s7.164,16,16,16s16-7.164,16-16S24.836,0,16,0z M10,10h12v12H10V10z" fill="currentColor" />
        </svg>
  );
};

