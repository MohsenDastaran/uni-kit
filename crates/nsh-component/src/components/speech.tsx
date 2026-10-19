// Dictation UI with an app-provided recognizer.
//
// NSH has no microphone or speech-recognizer API. The component owns the
// listening state and the transcript; the application supplies a recognizer
// (its own native module, a web bridge, or a script in a demo). This is a
// partial port and is recorded as such in docs/nsh-integration.md.
import type { StyleProp, ViewStyle } from 'nsh';
import { cleanup, effect, signal, Text, View } from 'nsh';
import { Button } from './button';
import { Icon } from './icon';
import { RADIUS_MD, tokens } from './theme';

export interface SpeechRecognizer {
  start?: (onTranscript: (text: string, final: boolean) => void) => void;
  stop?: () => void;
  available?: boolean;
}

export interface SpeechProps {
  recognizer?: SpeechRecognizer;
  transcript?: string;
  listening?: boolean;
  /** Show the live level bars above the transcript (default true). */
  waveform?: boolean;
  onTranscript?: (text: string, final: boolean) => void;
  onListeningChange?: (listening: boolean) => void;
  style?: StyleProp<ViewStyle>;
  testID?: string;
}

export function SpeechWaveform(props: { levels?: number[]; capturing?: boolean }) {
  const bars = props.levels ?? Array.from({ length: 18 }, () => 0.18);
  return (
    <View style={{ height: 30, flexDirection: 'row', alignItems: 'center', justifyContent: 'center', gap: 3, paddingHorizontal: 4 }}>
      {bars.map((level) => (
        <View style={{ width: 3, height: 3 + level * 24, borderRadius: 2, backgroundColor: props.capturing ? tokens.primary : tokens.mutedForeground, opacity: props.capturing ? 0.9 : 0.45 }} />
      ))}
    </View>
  );
}

export function Speech(props: SpeechProps) {
  const ownListening = signal(Boolean(props.listening));
  const ownTranscript = signal('');
  const levels = signal<number[]>(Array.from({ length: 18 }, () => 0.18));
  const listening = () => props.listening ?? ownListening();
  const transcript = () => props.transcript ?? ownTranscript();
  effect(() => {
    if (!listening()) {
      levels.set(Array.from({ length: 18 }, () => 0.18));
      return;
    }
    let tick = 0;
    const advance = () => {
      tick += 1;
      levels.set(
        Array.from({ length: 18 }, (_, index) => {
          const wave = Math.abs(Math.sin(tick * 0.55 + index * 0.62));
          const detail = Math.abs(Math.sin(tick * 0.21 + index * 1.7));
          return Math.max(0.14, Math.min(1, 0.18 + wave * 0.58 + detail * 0.2));
        }),
      );
    };
    advance();
    const timer = setInterval(advance, 90);
    cleanup(() => clearInterval(timer));
  });
  const start = () => {
    const recognizer = props.recognizer;
    const onText = (text: string, final: boolean) => {
      if (props.transcript === undefined) ownTranscript.set(text);
      props.onTranscript?.(text, final);
    };
    if (recognizer?.start) recognizer.start(onText);
    else {
      // No recognizer supplied: a short scripted transcript lets the UI be
      // exercised in the gallery without claiming microphone access.
      onText('This is a scripted transcript. The host app supplies the recognizer.', false);
      setTimeout(() => onText('This is a scripted transcript. The host app supplies the recognizer.', true), 900);
    }
    if (props.listening === undefined) ownListening.set(true);
    props.onListeningChange?.(true);
  };
  const stop = () => {
    props.recognizer?.stop?.();
    if (props.listening === undefined) ownListening.set(false);
    props.onListeningChange?.(false);
  };
  return (
    <View
      testID={props.testID}
      style={[
        {
          gap: 10,
          padding: 14,
          borderRadius: RADIUS_MD,
          borderWidth: 1,
          borderColor: listening() ? tokens.primary : tokens.border,
          backgroundColor: tokens.background,
        },
        props.style,
      ]}
    >
      <View style={{ flexDirection: 'row', alignItems: 'center', gap: 10 }}>
        <View
          style={{
            width: 40,
            height: 40,
            borderRadius: 20,
            alignItems: 'center',
            justifyContent: 'center',
            backgroundColor: listening() ? tokens.primary : tokens.muted,
          }}
        >
          <Icon name="mic" size={18} color={listening() ? tokens.primaryForeground : tokens.mutedForeground} />
        </View>
        <View style={{ flex: 1, gap: 2 }}>
          <Text style={{ fontSize: 14, fontWeight: '600', color: tokens.foreground }}>
            {listening() ? 'Listening…' : 'Dictate'}
          </Text>
          <Text style={{ fontSize: 12, color: tokens.mutedForeground }}>
            {props.recognizer?.available === false ? 'No recognizer available' : 'The host app supplies the recognizer.'}
          </Text>
        </View>
        <Button label={listening() ? 'Stop' : 'Start'} variant={listening() ? 'danger' : 'primary'} onPress={listening() ? stop : start} />
      </View>
      {props.waveform === false ? null : (
        <View
          accessibilityRole="progressbar"
          accessibilityLabel={listening() ? 'Microphone level' : 'Microphone idle'}
          style={{ height: 30, flexDirection: 'row', alignItems: 'center', justifyContent: 'center', gap: 3, paddingHorizontal: 4 }}
        >
          {levels().map((level) => (
            <View
              style={{
                width: 3,
                height: 3 + level * 24,
                borderRadius: 2,
                backgroundColor: listening() ? tokens.primary : tokens.mutedForeground,
                opacity: listening() ? 0.9 : 0.45,
              }}
            />
          ))}
        </View>
      )}
      <View style={{ minHeight: 48, padding: 10, borderRadius: RADIUS_MD, backgroundColor: tokens.muted }}>
        <Text style={{ fontSize: 13, color: transcript() ? tokens.foreground : tokens.mutedForeground }}>
          {transcript() || 'The transcript will appear here.'}
        </Text>
      </View>
    </View>
  );
}
