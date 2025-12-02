<!-- area-id: kaptara-island-east -->
### Kaptara Island (East)

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | ☑ |
| --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=charjabug">Charjabug</a> | 20% | 20% | <input type="checkbox" class="caught-check" data-species="charjabug" /> |
| <a href="../pokemon-lookup.html?q=cherrim">Cherrim</a> | 4% | — | <input type="checkbox" class="caught-check" data-species="cherrim" /> |
| <a href="../pokemon-lookup.html?q=dedenne">Dedenne</a> | 4% | — | <input type="checkbox" class="caught-check" data-species="dedenne" /> |
| <a href="../pokemon-lookup.html?q=doublade">Doublade</a> | 2% | 2% | <input type="checkbox" class="caught-check" data-species="doublade" /> |
| <a href="../pokemon-lookup.html?q=goomy">Goomy</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="goomy" /> |
| <a href="../pokemon-lookup.html?q=perrserker">Perrserker</a> | 5% | 5% | <input type="checkbox" class="caught-check" data-species="perrserker" /> |
| <a href="../pokemon-lookup.html?q=skiploom">Skiploom</a> | 20% | — | <input type="checkbox" class="caught-check" data-species="skiploom" /> |
| <a href="../pokemon-lookup.html?q=sligoo">Sligoo</a> | 5% | — | <input type="checkbox" class="caught-check" data-species="sligoo" /> |
| <a href="../pokemon-lookup.html?q=tauros">Tauros</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="tauros" /> |
| <a href="../pokemon-lookup.html?q=tauros-combat-breed">Tauros Combat Breed</a> | 10% | — | <input type="checkbox" class="caught-check" data-species="tauros-combat-breed" /> |
| <a href="../pokemon-lookup.html?q=weepinbell">Weepinbell</a> | 10% | 10% | <input type="checkbox" class="caught-check" data-species="weepinbell" /> |
| <a href="../pokemon-lookup.html?q=arbok">Arbok</a> | — | 20% | <input type="checkbox" class="caught-check" data-species="arbok" /> |
| <a href="../pokemon-lookup.html?q=hisuian-sligoo">Hisuian Sligoo</a> | — | 5% | <input type="checkbox" class="caught-check" data-species="hisuian-sligoo" /> |
| <a href="../pokemon-lookup.html?q=skuntank">Skuntank</a> | — | 4% | <input type="checkbox" class="caught-check" data-species="skuntank" /> |
| <a href="../pokemon-lookup.html?q=tauros-aqua-breed">Tauros Aqua Breed</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="tauros-aqua-breed" /> |
| <a href="../pokemon-lookup.html?q=tauros-blaze-breed">Tauros Blaze Breed</a> | — | 10% | <input type="checkbox" class="caught-check" data-species="tauros-blaze-breed" /> |
| <a href="../pokemon-lookup.html?q=togedemaru">Togedemaru</a> | — | 4% | <input type="checkbox" class="caught-check" data-species="togedemaru" /> |

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

