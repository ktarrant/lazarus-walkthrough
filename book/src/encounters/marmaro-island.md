<!-- area-id: marmaro-island -->
### Marmaro Island

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=anorith">Anorith</a> | 6% | 6% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="anorith" /> |
| <a href="../pokemon-lookup.html?q=dedenne">Dedenne</a> | 8% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="dedenne" /> |
| <a href="../pokemon-lookup.html?q=dwebble">Dwebble</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="dwebble" /> |
| <a href="../pokemon-lookup.html?q=growlithe">Growlithe</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="growlithe" /> |
| <a href="../pokemon-lookup.html?q=lileep">Lileep</a> | 6% | 6% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="lileep" /> |
| <a href="../pokemon-lookup.html?q=meditite">Meditite</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="meditite" /> |
| <a href="../pokemon-lookup.html?q=sizzlipede">Sizzlipede</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="sizzlipede" /> |
| <a href="../pokemon-lookup.html?q=swablu">Swablu</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="swablu" /> |
| <a href="../pokemon-lookup.html?q=tinkatink">Tinkatink</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="tinkatink" /> |
| <a href="../pokemon-lookup.html?q=aron">Aron</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="aron" /> |
| <a href="../pokemon-lookup.html?q=hisuian-growlithe">Hisuian Growlithe</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="hisuian-growlithe" /> |
| <a href="../pokemon-lookup.html?q=togedemaru">Togedemaru</a> | — | 8% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="togedemaru" /> |
| <a href="../pokemon-lookup.html?q=chinchou">Chinchou</a> | — | — | 60% | — | — | — | <input type="checkbox" class="caught-check" data-species="chinchou" /> |
| <a href="../pokemon-lookup.html?q=corsola">Corsola</a> | — | — | 5% | — | — | 15% | <input type="checkbox" class="caught-check" data-species="corsola" /> |
| <a href="../pokemon-lookup.html?q=finizen">Finizen</a> | — | — | 30% | — | — | — | <input type="checkbox" class="caught-check" data-species="finizen" /> |
| <a href="../pokemon-lookup.html?q=galarian-corsola">Galarian Corsola</a> | — | — | 5% | — | — | — | <input type="checkbox" class="caught-check" data-species="galarian-corsola" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | — | 30% | — | — | <input type="checkbox" class="caught-check" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=clauncher">Clauncher</a> | — | — | — | — | 20% | 40% | <input type="checkbox" class="caught-check" data-species="clauncher" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 20% | — | <input type="checkbox" class="caught-check" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | <input type="checkbox" class="caught-check" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | — | — | — | 40% | <input type="checkbox" class="caught-check" data-species="wailmer" /> |

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

