// A multi-step questionnaire with choices, freeform answers and navigation.
import type { StyleProp, ViewStyle } from 'nsh';
import { signal, Text, View } from 'nsh';
import { Button } from './button';
import { Input } from './input';
import { RADIUS_MD, tokens } from './theme';

export type QuestionnaireKind = 'choice' | 'text';

export interface QuestionnaireQuestion {
  id: string;
  title: string;
  description?: string;
  kind?: QuestionnaireKind;
  options?: { value: string; label: string }[];
  /** An extra free-text answer under the options. */
  other?: { label: string; placeholder?: string };
  required?: boolean;
}

export interface QuestionnaireProps {
  questions: QuestionnaireQuestion[];
  answers?: Record<string, string>;
  defaultAnswers?: Record<string, string>;
  onChange?: (answers: Record<string, string>) => void;
  onComplete?: (answers: Record<string, string>) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function Questionnaire(props: QuestionnaireProps) {
  const own = signal<Record<string, string>>(props.defaultAnswers ?? {});
  const step = signal(0);
  const answers = () => props.answers ?? own();
  const question = () => props.questions[step()];
  const setMany = (patch: Record<string, string>) => {
    const next = { ...answers(), ...patch };
    if (props.answers === undefined) own.set(next);
    props.onChange?.(next);
  };
  const set = (id: string, value: string) => setMany({ [id]: value });
  const canContinue = () => {
    const current = question();
    return !current?.required || Boolean(answers()[current.id]);
  };
  return (
    <View
      testID={props.testID}
      style={[
        {
          gap: 0,
          borderWidth: 1,
          borderColor: tokens.border,
          borderRadius: 12,
          backgroundColor: tokens.popover,
          overflow: 'hidden',
        },
        props.style,
      ]}
    >
      <View style={{ padding: 16, gap: 10 }}>
        <View style={{ flexDirection: 'row', alignItems: 'flex-start', gap: 10 }}>
          <Text style={{ flex: 1, fontSize: 16, lineHeight: 22, fontWeight: '600', color: tokens.foreground }}>
            {question()?.title ?? ''}
          </Text>
          <Text style={{ fontSize: 10, color: tokens.mutedForeground, paddingTop: 3 }}>
            Question {step() + 1} of {props.questions.length}
          </Text>
        </View>
        {question()?.description ? (
          <Text style={{ fontSize: 12, lineHeight: 18, color: tokens.mutedForeground }}>{question()!.description}</Text>
        ) : null}
        {question() ? (
          <View style={{ gap: 8 }}>
            {question()!.kind === 'text' ? (
              <Input
                value={answers()[question()!.id] ?? ''}
                onChangeText={(text) => set(question()!.id, text)}
                placeholder="Type your answer"
              />
            ) : (
              <>
                {(question()!.options ?? []).map((option) => {
                  const selected = answers()[question()!.id] === option.value;
                  return (
                    <View
                      accessibilityRole="radio"
                      accessibilityLabel={option.label}
                      accessibilityState={{ checked: selected }}
                      onPress={() => set(question()!.id, option.value)}
                      activeOpacity={0.85}
                      style={{
                        flexDirection: 'row',
                        alignItems: 'center',
                        gap: 9,
                        paddingHorizontal: 11,
                        paddingVertical: 9,
                        borderRadius: 9,
                        borderWidth: 1,
                        borderColor: selected ? tokens.primary : tokens.border,
                        backgroundColor: selected ? tokens.accent : tokens.background,
                      }}
                    >
                      <View
                        style={{
                          width: 16,
                          height: 16,
                          borderRadius: 8,
                          borderWidth: 1,
                          borderColor: selected ? tokens.primary : tokens.border,
                          alignItems: 'center',
                          justifyContent: 'center',
                        }}
                      >
                        {selected ? <View style={{ width: 8, height: 8, borderRadius: 4, backgroundColor: tokens.primary }} /> : null}
                      </View>
                      <Text style={{ flex: 1, fontSize: 12, color: tokens.foreground }}>{option.label}</Text>
                    </View>
                  );
                })}
                {question()!.other ? (
                  <View style={{ gap: 5, paddingTop: 2 }}>
                    <Text style={{ fontSize: 12, fontWeight: '500', color: tokens.foreground }}>{question()!.other!.label}</Text>
                    <Input
                      value={answers()[`${question()!.id}:other`] ?? ''}
                      onChangeText={(text) =>
                        setMany({
                          [`${question()!.id}:other`]: text,
                          [question()!.id]: text ? '__other__' : '',
                        })
                      }
                      placeholder={question()!.other!.placeholder ?? 'Tell us more'}
                    />
                  </View>
                ) : null}
              </>
            )}
          </View>
        ) : null}
      </View>
      <View
        style={{
          flexDirection: 'row',
          alignItems: 'center',
          justifyContent: step() > 0 ? 'space-between' : 'flex-end',
          gap: 8,
          paddingHorizontal: 16,
          paddingVertical: 12,
          borderTopWidth: 1,
          borderTopColor: tokens.border,
          backgroundColor: tokens.muted,
        }}
      >
        {step() > 0 ? (
          <Button label="Back" variant="ghost" size="small" onPress={() => step.update((value) => Math.max(0, value - 1))} />
        ) : null}
        <Button
          label={step() === props.questions.length - 1 ? 'Finish' : 'Next'}
          variant="primary"
          size="small"
          disabled={!canContinue()}
          onPress={() => {
            if (step() === props.questions.length - 1) props.onComplete?.(answers());
            else step.update((value) => Math.min(props.questions.length - 1, value + 1));
          }}
        />
      </View>
    </View>
  );
}
