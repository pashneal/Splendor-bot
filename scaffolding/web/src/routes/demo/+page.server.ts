import { fail } from '@sveltejs/kit';
import type { PageServerLoad} from './$types';

export const load = (({ cookies }) => {
  /// This function is called when the page is loaded (server-side)
}) satisfies PageServerLoad;

