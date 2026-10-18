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
  const set = (id: string, value: string) => {
    const next = { ...answers(), [id]: value };
    if (props.answers === undefined) own.set(next);
    props.onChange?.(next);
  };
  const canContinue = () => {
    const current = question();
    return !current?.required || Boolean(answers()[current.id]);
  };
  return (
    <View testID={props.testID} style={[{ gap: 14, padding: 16, borderWidth: 1, borderColor: tokens.border, borderRadius: RADIUS_MD }, props.style]}>
      <View style={{ flexDirection: 'row', justifyContent: 'space-between' }}>
        <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>Question {step() + 1} of {props.questions.length}</Text>
        <View style={{ flexDirection: 'row', gap: 4 }}>
          {props.questions.map((_, index) => (
            <View style={{ width: 6, height: 6, borderRadius: 3, backgroundColor: index <= step() ? tokens.primary : tokens.border }} />
          ))}
        </View>
      </View>
      {question() ? (
        <View style={{ gap: 10 }}>
          <Text style={{ fontSize: 16, fontWeight: '600', color: tokens.foreground }}>{question().title}</Text>
          {question().description ? <Text style={{ fontSize: 13, color: tokens.mutedForeground }}>{question().description}</Text> : null}
          {question().kind === 'text' ? (
            <Input value={answers()[question().id] ?? ''} onChangeText={(text) => set(question().id, text)} placeholder="Type your answer" />
          ) : (
            <View style={{ gap: 6 }}>
              {(question().options ?? []).map((option) => {
                const selected = answers()[question().id] === option.value;
                return (
                  <View
                    accessibilityRole="radio"
                    accessibilityLabel={option.label}
                    accessibilityState={{ checked: selected }}
                    onPress={() => set(question().id, option.value)}
                    activeOpacity={0.8}
                    style={{ flexDirection: 'row', alignItems: 'center', gap: 8, padding: 9, borderRadius: RADIUS_MD, borderWidth: 1, borderColor: selected ? tokens.primary : tokens.border, backgroundColor: selected ? tokens.accent : tokens.background }}
                  >
                    <Text style={{ fontSize: 13, color: tokens.foreground }}>{option.label}</Text>
                  </View>
                );
              })}
            </View>
          )}
        </View>
      ) : null}
      <View style={{ flexDirection: 'row', justifyContent: 'space-between', marginTop: 4 }}>
        <Button label="Back" variant="ghost" disabled={step() === 0} onPress={() => step.update((value) => Math.max(0, value - 1))} />
        <Button
          label={step() === props.questions.length - 1 ? 'Finish' : 'Next'}
          variant="primary"
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
