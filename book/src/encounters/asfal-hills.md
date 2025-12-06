<!-- area-id: asfal-hills -->
### Asfal Hills

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=buizel">Buizel</a> | 5% | — | <input type="checkbox" class="caught-check" data-species="buizel" /> |
| <a href="../pokemon-lookup.html?q=chatot">Chatot</a> | 6% | — | <input type="checkbox" class="caught-check" data-species="chatot" /> |
| <a href="../pokemon-lookup.html?q=galarian-meowth">Galarian Meowth</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="galarian-meowth" /> |
| <a href="../pokemon-lookup.html?q=hawlucha">Hawlucha</a> | 4% | 4% | <input type="checkbox" class="caught-check" data-species="hawlucha" /> |
| <a href="../pokemon-lookup.html?q=pachirisu">Pachirisu</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="pachirisu" /> |
| <a href="../pokemon-lookup.html?q=pawmi">Pawmi</a> | 15% | 15% | <input type="checkbox" class="caught-check" data-species="pawmi" /> |
| <a href="../pokemon-lookup.html?q=tauros">Tauros</a> | 20% | — | <input type="checkbox" class="caught-check" data-species="tauros" /> |
| <a href="../pokemon-lookup.html?q=paldean-tauros-c">Tauros Combat Breed</a> | 20% | — | <input type="checkbox" class="caught-check" data-species="paldean-tauros-c" /> |
| <a href="../pokemon-lookup.html?q=yamper">Yamper</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="yamper" /> |
| <a href="../pokemon-lookup.html?q=dedenne">Dedenne</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="dedenne" /> |
| <a href="../pokemon-lookup.html?q=shroodle">Shroodle</a> | — | 5% | <input type="checkbox" class="caught-check" data-species="shroodle" /> |
| <a href="../pokemon-lookup.html?q=stunky">Stunky</a> | — | 6% | <input type="checkbox" class="caught-check" data-species="stunky" /> |
| <a href="../pokemon-lookup.html?q=paldean-tauros-b">Tauros Aqua Breed</a> | — | 20% | <input type="checkbox" class="caught-check" data-species="paldean-tauros-b" /> |
| <a href="../pokemon-lookup.html?q=paldean-tauros-a">Tauros Blaze Breed</a> | — | 20% | <input type="checkbox" class="caught-check" data-species="paldean-tauros-a" /> |

<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>

