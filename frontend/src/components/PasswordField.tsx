import { useState } from 'react'
import IconButton from '@mui/material/IconButton'
import InputAdornment from '@mui/material/InputAdornment'
import TextField, { type TextFieldProps } from '@mui/material/TextField'
import Visibility from '@mui/icons-material/Visibility'
import VisibilityOff from '@mui/icons-material/VisibilityOff'

/**
 * Password fields with an eye toggle at the right edge that reveals what has been typed.
 *
 * There are two shapes because the app has two kinds of form: `PasswordTextField` for the MUI
 * dialogs, and `PasswordInput` for the plain-HTML auth pages (login, register, reset/change
 * password). Visibility is per-field state and always starts hidden, including after a remount.
 */

const toggleLabel = (visible: boolean) => (visible ? 'Hide password' : 'Show password')

/** Drop-in for a `<TextField type="password">`; takes every TextField prop. */
export function PasswordTextField(props: TextFieldProps) {
  const [visible, setVisible] = useState(false)

  return (
    <TextField
      {...props}
      type={visible ? 'text' : 'password'}
      InputProps={{
        ...props.InputProps,
        endAdornment: (
          <InputAdornment position="end">
            <IconButton
              size="small"
              edge="end"
              onClick={() => setVisible((v) => !v)}
              aria-label={toggleLabel(visible)}
              aria-pressed={visible}
              title={toggleLabel(visible)}
            >
              {visible ? <VisibilityOff fontSize="small" /> : <Visibility fontSize="small" />}
            </IconButton>
          </InputAdornment>
        ),
      }}
    />
  )
}

/** Drop-in for a bare `<input type="password">` in the `.Form` auth pages. */
export function PasswordInput({
  style,
  ...props
}: Omit<React.InputHTMLAttributes<HTMLInputElement>, 'type'>) {
  const [visible, setVisible] = useState(false)

  return (
    // The toggle is positioned against the input, so the wrapper takes over the bottom margin
    // that `.Form input` would otherwise open up between the two.
    <span style={{ position: 'relative', display: 'block', marginBottom: 20 }}>
      <input
        {...props}
        type={visible ? 'text' : 'password'}
        style={{
          display: 'block',
          width: '100%',
          boxSizing: 'border-box',
          paddingRight: 40,
          marginBottom: 0,
          ...style,
        }}
      />
      <button
        type="button"
        onClick={() => setVisible((v) => !v)}
        aria-label={toggleLabel(visible)}
        aria-pressed={visible}
        title={toggleLabel(visible)}
        // Inline, because `.Form button` in App.css would otherwise pad this like a submit button.
        style={{
          position: 'absolute',
          top: 0,
          right: 0,
          height: '100%',
          width: 38,
          padding: 0,
          border: 'none',
          background: 'none',
          cursor: 'pointer',
          color: '#555',
          display: 'flex',
          alignItems: 'center',
          justifyContent: 'center',
        }}
      >
        {visible ? <VisibilityOff fontSize="small" /> : <Visibility fontSize="small" />}
      </button>
    </span>
  )
}
