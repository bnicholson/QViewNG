import { DataTableTemplate, type ColumnDef } from './DataTableTemplate';

interface RoomMonitorRow {
  id: number;
  bldgroom: string;
  chkdin: string;
  tournament: string;
  division: string;
  room: string;
  round: string;
  question: number;
  hostip: string;
  qmversion: string;
  pending: number;
  status_error: string;
  resend: string;
}

const columns: ColumnDef<RoomMonitorRow>[] = [
  { header: 'BldgRoom',     render: (r) => r.bldgroom     },
  { header: 'ChkdIn',       render: (r) => r.chkdin       },
  { header: 'Tournament',   render: (r) => r.tournament   },
  { header: 'Division',     render: (r) => r.division     },
  { header: 'Room',         render: (r) => r.room         },
  { header: 'Round',        render: (r) => r.round        },
  { header: 'Question',     render: (r) => r.question     },
  { header: 'Host/IP',      render: (r) => r.hostip       },
  { header: 'QMVersion',    render: (r) => r.qmversion    },
  { header: 'Pending',      render: (r) => r.pending      },
  { header: 'Status/Error', render: (r) => r.status_error },
  { header: 'Resend',       render: (r) => r.resend       },
];

// Placeholder rows matching the legacy RoomMonitor component
const PLACEHOLDER_ROWS: RoomMonitorRow[] = Array.from({ length: 5 }, (_, i) => ({
  id: i,
  bldgroom: 'Jester 103',
  chkdin: '11:03',
  tournament: 'Q2022',
  division: 'Local-Experienced',
  room: 'Jester 103',
  round: 'Tues07d',
  question: 21,
  hostip: '192.168.4.23',
  qmversion: '5.4 J30',
  pending: 33,
  status_error: 'Missing a quizzer',
  resend: 'resend 33',
}));

export default function RoomMonitorTable({ tid: _tid }: { tid: string }) {
  return (
    <DataTableTemplate<RoomMonitorRow>
      entityLabel="Room Monitor"
      columns={columns}
      rows={PLACEHOLDER_ROWS}
      totalCount={PLACEHOLDER_ROWS.length}
      getId={(r) => String(r.id)}
      page={0}
      pageSize={PLACEHOLDER_ROWS.length}
      onPageChange={() => {}}
      onPageSizeChange={() => {}}
    />
  );
}
