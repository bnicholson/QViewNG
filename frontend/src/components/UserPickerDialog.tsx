import React, { useCallback, useEffect, useState } from 'react'
import AppBar from '@mui/material/AppBar'
import Alert from '@mui/material/Alert'
import Box from '@mui/material/Box'
import Button from '@mui/material/Button'
import CloseIcon from '@mui/icons-material/Close'
import Collapse from '@mui/material/Collapse'
import Dialog from '@mui/material/Dialog'
import IconButton from '@mui/material/IconButton'
import Slide from '@mui/material/Slide'
import TextField from '@mui/material/TextField'
import Toolbar from '@mui/material/Toolbar'
import Typography from '@mui/material/Typography'
import { type TransitionProps } from '@mui/material/transitions'
import { UserAPI, type UserTS } from '../features/UserAPI'

const Transition = React.forwardRef(function Transition(
  props: TransitionProps & { children: React.ReactElement },
  ref: React.Ref<unknown>,
) {
  return <Slide direction="up" ref={ref} {...props} />;
});

interface Props {
  isOpen: boolean;
  title: string;
  /** User IDs already in the target (hidden from the list) */
  excludeIds: string[];
  onCancel: VoidFunction;
  onPick: (user: UserTS) => Promise<void>;
  /** When provided, only these users are shown instead of fetching all users from the API. */
  availableUsers?: UserTS[];
}

export const UserPickerDialog = ({ isOpen, title, excludeIds, onCancel, onPick, availableUsers }: Props) => {
  const [fetchedUsers, setFetchedUsers] = useState<UserTS[]>([]);
  const [filter, setFilter] = useState('');
  const [adding, setAdding] = useState<string | null>(null);
  const [error, setError] = useState('');

  const loadUsers = useCallback(async () => {
    if (availableUsers) return; // Skip fetch when caller provides the list
    try {
      const result = await UserAPI.get(0, 500);
      setFetchedUsers(result.items);
    } catch {
      setError('Failed to load users.');
    }
  }, [availableUsers]);

  useEffect(() => {
    if (isOpen) {
      setFilter('');
      setError('');
      setAdding(null);
      loadUsers();
    }
  }, [isOpen, loadUsers]);

  const allUsers = availableUsers ?? fetchedUsers;
  const excludeSet = new Set(excludeIds);
  const lowerFilter = filter.toLowerCase();
  const filtered = allUsers.filter(u => {
    if (excludeSet.has(u.id)) return false;
    if (!lowerFilter) return true;
    const fullName = `${u.fname} ${u.mname} ${u.lname}`.toLowerCase();
    return fullName.includes(lowerFilter)
      || u.username.toLowerCase().includes(lowerFilter)
      || u.email.toLowerCase().includes(lowerFilter);
  });

  const handlePick = async (user: UserTS) => {
    setAdding(user.id);
    setError('');
    try {
      await onPick(user);
    } catch (err: any) {
      setError(err.message ?? 'Failed to add user.');
    } finally {
      setAdding(null);
    }
  };

  return (
    <Dialog fullScreen open={isOpen} onClose={onCancel} slots={{ transition: Transition }}>
      <AppBar sx={{ position: 'sticky' }}>
        <Toolbar>
          <IconButton edge="start" color="inherit" onClick={onCancel} aria-label="close">
            <CloseIcon />
          </IconButton>
          <Typography sx={{ ml: 2, flex: 1 }} variant="h6" component="div">
            {title}
          </Typography>
        </Toolbar>
      </AppBar>

      <Box sx={{ maxWidth: 900, mx: 'auto', width: '100%', p: 2 }}>
        <Collapse in={!!error}>
          <Alert severity="error" sx={{ mb: 2 }} onClose={() => setError('')}>
            {error}
          </Alert>
        </Collapse>

        <TextField
          placeholder="Search by name, username, or email..."
          value={filter}
          onChange={e => setFilter(e.target.value)}
          fullWidth
          variant="outlined"
          size="small"
          sx={{ mb: 2 }}
          autoFocus
        />

        <Box sx={{ overflowY: 'auto', maxHeight: 'calc(100vh - 180px)', borderRadius: 2, border: '1px solid #e5e7eb' }}>
          <table style={{ width: '100%', borderCollapse: 'collapse', fontSize: 14 }}>
            <thead>
              <tr style={{ background: '#f9fafb', borderBottom: '1px solid #e5e7eb', position: 'sticky', top: 0, zIndex: 1 }}>
                <th style={{ padding: '8px 14px', textAlign: 'left', fontWeight: 600, fontSize: 12, color: '#6b7280', textTransform: 'uppercase' }}>Name</th>
                <th style={{ padding: '8px 14px', textAlign: 'left', fontWeight: 600, fontSize: 12, color: '#6b7280', textTransform: 'uppercase' }}>Username</th>
                <th style={{ padding: '8px 14px', textAlign: 'left', fontWeight: 600, fontSize: 12, color: '#6b7280', textTransform: 'uppercase' }}>Email</th>
                <th style={{ padding: '8px 14px', width: 80 }} />
              </tr>
            </thead>
            <tbody>
              {filtered.length === 0 ? (
                <tr>
                  <td colSpan={4} style={{ padding: '32px 14px', textAlign: 'center', color: '#9ca3af' }}>
                    {allUsers.length === 0 ? 'Loading...' : 'No matching users found.'}
                  </td>
                </tr>
              ) : (
                filtered.map((u, i) => (
                  <tr
                    key={u.id}
                    style={{
                      background: i % 2 === 0 ? '#fff' : '#fafafa',
                      borderBottom: '1px solid #f3f4f6',
                    }}
                  >
                    <td style={{ padding: '8px 14px', color: '#374151', fontWeight: 500 }}>
                      {`${u.fname} ${u.mname ? u.mname + ' ' : ''}${u.lname}`}
                    </td>
                    <td style={{ padding: '8px 14px', color: '#6b7280' }}>{u.username}</td>
                    <td style={{ padding: '8px 14px', color: '#6b7280' }}>{u.email}</td>
                    <td style={{ padding: '8px 14px' }}>
                      <Button
                        size="small"
                        variant="contained"
                        disabled={adding === u.id}
                        onClick={() => handlePick(u)}
                        sx={{ textTransform: 'none', fontSize: 12, minWidth: 60 }}
                      >
                        {adding === u.id ? 'Adding...' : 'Add'}
                      </Button>
                    </td>
                  </tr>
                ))
              )}
            </tbody>
          </table>
        </Box>

        <Typography variant="body2" color="text.secondary" sx={{ mt: 1, textAlign: 'right' }}>
          {filtered.length} user{filtered.length !== 1 ? 's' : ''} shown
        </Typography>
      </Box>
    </Dialog>
  );
};
