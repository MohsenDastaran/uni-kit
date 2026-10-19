// A step-by-step process indicator: numbered, dotted, card, panel, pill, arrow
// or vertical rail layouts, with optional Previous/Next navigation.
import type { JSX, StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Button } from './button';
import { Icon } from './icon';
import { type IconName } from './icons';
import { type ControlSize, RADIUS_MD, tokens } from './theme';

export type StepperStatus = 'complete' | 'current' | 'upcoming' | 'error';
export type StepperVariant = 'number' | 'dots' | 'cards' | 'panel' | 'pills' | 'arrows' | 'rail';

export interface StepperItem {
  id?: string;
  label: string;
  description?: string;
  icon?: IconName;
  disabled?: boolean;
  /** Overrides the status derived from the active index. */
  status?: StepperStatus;
  /** A short footer value, such as a time or owner. */
  meta?: string;
  /** A per-step accent, used by the panel variant. */
  color?: string;
}

export interface StepperProps {
  items?: StepperItem[];
  steps?: StepperItem[];
  selectedIndex?: number;
  active?: number;
  onChange?: (index: number) => void;
  onStepPress?: (index: number) => void;
  vertical?: boolean;
  orientation?: 'horizontal' | 'vertical';
  variant?: StepperVariant;
  size?: ControlSize;
  textCenter?: boolean;
  /** Render Previous/Next controls under the steps. */
  showNavigation?: boolean;
  previousLabel?: string;
  nextLabel?: string;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

const sizeOf = (size: ControlSize | undefined): number =>
  size === 'xsmall' ? 18 : size === 'small' ? 22 : size === 'large' ? 34 : 28;

const iconOf = (size: ControlSize | undefined): number =>
  size === 'xsmall' ? 9 : size === 'small' ? 11 : size === 'large' ? 16 : 13;

export function Stepper(props: StepperProps) {
  const own = signal(props.selectedIndex ?? props.active ?? 0);
  const current = () => props.selectedIndex ?? props.active ?? own();
  const steps = () => props.items ?? props.steps ?? [];
  const vertical = () => props.vertical ?? props.orientation === 'vertical';
  const variant = () => props.variant ?? 'number';
  const centered = () => Boolean(props.textCenter) || (!vertical() && variant() !== 'rail');
  const statusOf = (step: StepperItem, index: number): StepperStatus =>
    step.status ?? (index < current() ? 'complete' : index === current() ? 'current' : 'upcoming');
  const tone = (status: StepperStatus) =>
    status === 'complete' ? tokens.success : status === 'current' ? tokens.primary : status === 'error' ? tokens.danger : tokens.mutedForeground;
  const select = (index: number, step: StepperItem) => {
    if (step.disabled) return;
    if (props.selectedIndex === undefined && props.active === undefined) own.set(index);
    (props.onStepPress ?? props.onChange)?.(index);
  };

  const indicator = (step: StepperItem, index: number, status: StepperStatus) => {
    const size = sizeOf(props.size);
    const done = status === 'complete';
    const current = status === 'current';
    const accent = tone(status);
    if (variant() === 'dots') {
      return (
        <View style={{ width: size < 24 ? 10 : 12, height: size < 24 ? 10 : 12, borderRadius: 99, backgroundColor: done ? tokens.success : current ? tokens.primary : tokens.muted, borderWidth: current ? 2 : 0, borderColor: tokens.background, opacity: status === 'upcoming' ? 0.55 : 1 }} />
      );
    }
    return (
      <View
        style={{
          width: size,
          height: size,
          borderRadius: 99,
          alignItems: 'center',
          justifyContent: 'center',
          backgroundColor: done ? tokens.success : current ? tokens.background : tokens.background,
          borderWidth: done ? 0 : 1,
          borderColor: status === 'error' ? tokens.danger : current ? tokens.primary : tokens.border,
          opacity: status === 'upcoming' ? 0.7 : 1,
          flexShrink: 0,
        }}
      >
        {done ? (
          <Icon name="check" size={iconOf(props.size)} color={tokens.primaryForeground} />
        ) : step.icon ? (
          <Icon name={step.icon} size={iconOf(props.size)} color={current ? tokens.foreground : tokens.mutedForeground} />
        ) : (
          <Text style={{ fontSize: props.size === 'large' ? 13 : 10, fontWeight: '600', color: current ? tokens.foreground : tokens.mutedForeground }}>{index + 1}</Text>
        )}
      </View>
    );
  };

  const copy = (step: StepperItem, align: 'center' | 'left') => (
    <View style={{ gap: 2, alignItems: align === 'center' ? 'center' : 'flex-start' }}>
      <Text style={{ fontSize: props.size === 'large' ? 14 : 12, fontWeight: '500', color: tokens.foreground, textAlign: align === 'center' ? 'center' : 'left' }}>{step.label}</Text>
      {step.description ? (
        <Text style={{ fontSize: 10, lineHeight: 14, color: tokens.mutedForeground, textAlign: align === 'center' ? 'center' : 'left' }}>{step.description}</Text>
      ) : null}
      {step.meta ? <Text style={{ fontSize: 9, color: tokens.mutedForeground, textAlign: align === 'center' ? 'center' : 'left' }}>{step.meta}</Text> : null}
    </View>
  );

  const stepRow = (step: StepperItem, index: number, last: boolean) => {
    const status = statusOf(step, index);
    const accent = tone(status);
    const align: 'center' | 'left' = centered() ? 'center' : 'left';
    return (
      <View
        accessibilityRole="button"
        accessibilityLabel={step.label}
        accessibilityState={{ selected: index === current(), disabled: Boolean(step.disabled) }}
        onPress={() => select(index, step)}
        activeOpacity={0.85}
        style={{ flex: 1, minWidth: 0, flexDirection: 'row', alignItems: 'flex-start', gap: 8, opacity: step.disabled ? 0.5 : 1 }}
      >
        <View style={{ flex: 1, minWidth: 0, alignItems: align === 'center' ? 'center' : 'flex-start', gap: 6 }}>
          <View style={{ width: '100%', flexDirection: 'row', alignItems: 'center' }}>
            {indicator(step, index, status)}
            {!last ? (
              <View style={{ flex: 1, height: 2, marginHorizontal: 8, borderRadius: 1, backgroundColor: status === 'complete' ? tokens.success : tokens.border }} />
            ) : null}
          </View>
          {variant() === 'dots' ? null : copy(step, align)}
        </View>
      </View>
    );
  };

  if (variant() === 'cards') {
    return (
      <View testID={props.testID} style={[{ width: '100%', gap: 8 }, props.style]}>
        <View style={{ flexDirection: vertical() ? 'column' : 'row', gap: 8 }}>
          {steps().map((step, index) => {
            const status = statusOf(step, index);
            const selected = index === current();
            return (
              <View
                accessibilityRole="button"
                accessibilityLabel={step.label}
                accessibilityState={{ selected, disabled: Boolean(step.disabled) }}
                onPress={() => select(index, step)}
                activeOpacity={0.85}
                style={{
                  flex: 1,
                  minWidth: 0,
                  flexDirection: 'row',
                  alignItems: 'center',
                  gap: 9,
                  padding: 11,
                  borderRadius: 9,
                  borderWidth: 1,
                  borderColor: selected ? tokens.primary : tokens.border,
                  backgroundColor: selected ? tokens.accent : tokens.popover,
                  opacity: step.disabled ? 0.5 : 1,
                }}
              >
                {indicator(step, index, status)}
                <View style={{ flex: 1, minWidth: 0 }}>{copy(step, 'left')}</View>
                <Icon name={status === 'complete' ? 'check' : status === 'current' ? 'loader-circle' : 'circle'} size={12} color={tone(status)} />
              </View>
            );
          })}
        </View>
        {navigation()}
      </View>
    );
  }

  if (variant() === 'panel') {
    return (
      <View testID={props.testID} style={[{ width: '100%', gap: 10 }, props.style]}>
        <View style={{ flexDirection: 'row', borderWidth: 1, borderColor: tokens.border, borderRadius: 9, overflow: 'hidden', backgroundColor: tokens.popover }}>
          {steps().map((step, index) => {
            const status = statusOf(step, index);
            return (
              <View accessibilityRole="button" accessibilityLabel={step.label} onPress={() => select(index, step)} style={{ flex: 1, minWidth: 0, paddingTop: 10, paddingHorizontal: 10, gap: 5, borderBottomWidth: 3, borderBottomColor: step.color ?? tone(status), backgroundColor: index === current() ? tokens.accent : 'transparent' }}>
                <View style={{ flexDirection: 'row', alignItems: 'center', gap: 5 }}>
                  {indicator(step, index, status)}
                  <Text style={{ flex: 1, fontSize: 11, fontWeight: '500', color: index === current() ? tokens.foreground : tokens.mutedForeground }}>{step.label}</Text>
                </View>
                {step.description ? <Text style={{ fontSize: 9, lineHeight: 12, color: tokens.mutedForeground, paddingBottom: 8 }}>{step.description}</Text> : <View style={{ height: 8 }} />}
              </View>
            );
          })}
        </View>
        {navigation()}
      </View>
    );
  }

  if (variant() === 'pills' || variant() === 'arrows') {
    const arrows = variant() === 'arrows';
    return (
      <View testID={props.testID} style={[{ width: '100%', gap: 10 }, props.style]}>
        <View style={{ flexDirection: 'row', alignItems: 'center', flexWrap: 'wrap', gap: arrows ? 6 : 5 }}>
          {steps().map((step, index) => {
            const status = statusOf(step, index);
            const selected = index === current();
            const done = status === 'complete';
            return (
              <View style={{ flexDirection: 'row', alignItems: 'center', gap: 5 }}>
                <View
                  accessibilityRole="button"
                  accessibilityLabel={step.label}
                  accessibilityState={{ selected }}
                  onPress={() => select(index, step)}
                  style={{
                    flexDirection: 'row',
                    alignItems: 'center',
                    gap: 5,
                    paddingHorizontal: arrows ? 10 : 11,
                    paddingVertical: 6,
                    borderRadius: arrows ? 5 : 999,
                    borderWidth: 1,
                    borderColor: done ? tokens.success : selected ? tokens.primary : tokens.border,
                    backgroundColor: selected ? tokens.primary : done ? tokens.accent : tokens.popover,
                  }}
                >
                  <Icon name={done ? 'check' : selected ? 'loader-circle' : 'circle'} size={10} color={selected ? tokens.primaryForeground : done ? tokens.success : tokens.mutedForeground} />
                  <Text style={{ fontSize: 10, fontWeight: '500', color: selected ? tokens.primaryForeground : done ? tokens.success : tokens.foreground }}>{step.label}</Text>
                </View>
                {index < steps().length - 1 ? <Icon name="chevron-right" size={11} color={tokens.mutedForeground} /> : null}
              </View>
            );
          })}
        </View>
        {navigation()}
      </View>
    );
  }

  if (vertical()) {
    return (
      <View testID={props.testID} style={[{ width: '100%', gap: 0 }, props.style]}>
        {steps().map((step, index) => {
          const status = statusOf(step, index);
          const selected = index === current();
          const last = index === steps().length - 1;
          return (
            <View
              accessibilityRole="button"
              accessibilityLabel={step.label}
              accessibilityState={{ selected, disabled: Boolean(step.disabled) }}
              onPress={() => select(index, step)}
              activeOpacity={0.85}
              style={{ flexDirection: 'row', alignItems: 'flex-start', gap: 10, padding: 7, borderRadius: 7, backgroundColor: selectionSurface(variant(), selected) }}
            >
              <View style={{ alignItems: 'center', gap: 3 }}>
                {indicator(step, index, status)}
                {!last ? <View style={{ width: 2, flex: 1, minHeight: 24, borderRadius: 1, backgroundColor: status === 'complete' ? tokens.success : tokens.border }} /> : null}
              </View>
              <View style={{ flex: 1, paddingTop: 2, paddingBottom: last ? 0 : 12 }}>{copy(step, 'left')}</View>
              {variant() === 'rail' ? (
                <View style={{ width: 2, height: 22, borderRadius: 1, backgroundColor: selected ? tokens.primary : tokens.border, marginTop: 4 }} />
              ) : null}
            </View>
          );
        })}
        {navigation()}
      </View>
    );
  }

  return (
    <View testID={props.testID} style={[{ width: '100%', gap: 10 }, props.style]}>
      <View style={{ flexDirection: 'row', alignItems: 'flex-start', width: '100%' }}>
        {steps().map((step, index) => stepRow(step, index, index === steps().length - 1))}
      </View>
      {navigation()}
    </View>
  );

  function navigation(): JSX.Element | null {
    if (!props.showNavigation && !props.onChange) return null;
    if (!props.showNavigation) return null;
    return (
      <View style={{ flexDirection: 'row', alignItems: 'center', justifyContent: 'center', gap: 8, paddingTop: 2 }}>
        <Button label={props.previousLabel ?? 'Previous'} size="small" variant="secondary" disabled={current() <= 0} onPress={() => select(Math.max(0, current() - 1), steps()[current() - 1] ?? steps()[0])} />
        <Button label={props.nextLabel ?? 'Next'} size="small" variant="primary" disabled={current() >= steps().length - 1} onPress={() => select(Math.min(steps().length - 1, current() + 1), steps()[current() + 1] ?? steps()[steps().length - 1])} />
      </View>
    );
  }
}

function selectionSurface(variant: StepperVariant, selected: boolean): string {
  return variant === 'rail' && selected ? tokens.accent : 'transparent';
}

/** A compact detail block for the selected step, used under a Stepper. */
export function StepperDetail(props: { title: string; description?: string; eyebrow?: string; children?: JSX.Element; style?: StyleProp<ViewStyle> }) {
  return (
    <View style={[{ width: '100%', gap: 4, paddingTop: 4 }, props.style]}>
      {props.eyebrow ? <Text style={{ fontSize: 9, color: tokens.mutedForeground }}>{props.eyebrow}</Text> : null}
      <Text style={{ fontSize: 13, fontWeight: '600', color: tokens.foreground }}>{props.title}</Text>
      {props.description ? <Text style={{ fontSize: 11, lineHeight: 15, color: tokens.mutedForeground }}>{props.description}</Text> : null}
      {props.children}
    </View>
  );
}
