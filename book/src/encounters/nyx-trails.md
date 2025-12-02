<!-- area-id: nyx-trails -->
### Nyx Trails

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=abomasnow">Abomasnow</a> | 5% | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="abomasnow" /> |
| <a href="../pokemon-lookup.html?q=banette">Banette</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="banette" /> |
| <a href="../pokemon-lookup.html?q=breloom">Breloom</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="breloom" /> |
| <a href="../pokemon-lookup.html?q=dusclops">Dusclops</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="dusclops" /> |
| <a href="../pokemon-lookup.html?q=gogoat">Gogoat</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="gogoat" /> |
| <a href="../pokemon-lookup.html?q=hisuian-electrode">Hisuian Electrode</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="hisuian-electrode" /> |
| <a href="../pokemon-lookup.html?q=lurantis">Lurantis</a> | 5% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="lurantis" /> |
| <a href="../pokemon-lookup.html?q=perrserker">Perrserker</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="perrserker" /> |
| <a href="../pokemon-lookup.html?q=persian">Persian</a> | 10% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="persian" /> |
| <a href="../pokemon-lookup.html?q=snover">Snover</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="snover" /> |
| <a href="../pokemon-lookup.html?q=xatu">Xatu</a> | 20% | — | — | — | — | — | <input type="checkbox" class="caught-check" data-species="xatu" /> |
| <a href="../pokemon-lookup.html?q=alolan-persian">Alolan Persian</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="alolan-persian" /> |
| <a href="../pokemon-lookup.html?q=duskull">Duskull</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="duskull" /> |
| <a href="../pokemon-lookup.html?q=gourgeist">Gourgeist</a> | — | 5% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="gourgeist" /> |
| <a href="../pokemon-lookup.html?q=pumpkaboo">Pumpkaboo</a> | — | 20% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="pumpkaboo" /> |
| <a href="../pokemon-lookup.html?q=shuppet">Shuppet</a> | — | 10% | — | — | — | — | <input type="checkbox" class="caught-check" data-species="shuppet" /> |
| <a href="../pokemon-lookup.html?q=bruxish">Bruxish</a> | — | — | 10% | — | — | 20% | <input type="checkbox" class="caught-check" data-species="bruxish" /> |
| <a href="../pokemon-lookup.html?q=lumineon">Lumineon</a> | — | — | 30% | — | — | — | <input type="checkbox" class="caught-check" data-species="lumineon" /> |
| <a href="../pokemon-lookup.html?q=tentacruel">Tentacruel</a> | — | — | 60% | — | — | — | <input type="checkbox" class="caught-check" data-species="tentacruel" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="caught-check" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=shellder">Shellder</a> | — | — | — | 30% | 20% | — | <input type="checkbox" class="caught-check" data-species="shellder" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | 40% | <input type="checkbox" class="caught-check" data-species="clamperl" /> |
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

