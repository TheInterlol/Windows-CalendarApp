import { createClient } from '@supabase/supabase-js';

// Tvoje URL složená z Project ID
const supabaseUrl = 'https://vpjdunzhwigoybdpggid.supabase.co'; 

// Tvůj klíč z minulého screenshotu (celý ten řádek, co začínal na sb_publishable...)
// (Pozor, doplň si tam zbytek toho klíče, na fotce byl uříznutý!)
const supabaseKey = 'sb_publishable_M8Q2drPX3uiOhpBeoV12UA_Fdnug8yo'; 

export const supabase = createClient(supabaseUrl, supabaseKey);