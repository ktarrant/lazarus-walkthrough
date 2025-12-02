<!-- area-id: fresco-isles -->
### Fresco Isles

_Source: Pokemon Lazarus Encounters PDF_

| Pokémon | Grass (Day) | Grass (Night) | Surfing | Old Rod | Good Rod | Super Rod | ☑ |
| --- | --- | --- | --- | --- | --- | --- | --- |
| <a href="../pokemon-lookup.html?q=cyclizar">Cyclizar</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="cyclizar" /> |
| <a href="../pokemon-lookup.html?q=doduo">Doduo</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="doduo" /> |
| <a href="../pokemon-lookup.html?q=gligar">Gligar</a> | 10% | 10% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="gligar" /> |
| <a href="../pokemon-lookup.html?q=helioptile">Helioptile</a> | 5% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="helioptile" /> |
| <a href="../pokemon-lookup.html?q=krokorok">Krokorok</a> | 5% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="krokorok" /> |
| <a href="../pokemon-lookup.html?q=larvitar">Larvitar</a> | 2% | 2% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="larvitar" /> |
| <a href="../pokemon-lookup.html?q=numel">Numel</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="numel" /> |
| <a href="../pokemon-lookup.html?q=scraggy">Scraggy</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="scraggy" /> |
| <a href="../pokemon-lookup.html?q=skorupi">Skorupi</a> | 4% | 4% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="skorupi" /> |
| <a href="../pokemon-lookup.html?q=tinkatuff">Tinkatuff</a> | 10% | — | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="tinkatuff" /> |
| <a href="../pokemon-lookup.html?q=trapinch">Trapinch</a> | 20% | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="trapinch" /> |
| <a href="../pokemon-lookup.html?q=dreepy">Dreepy</a> | — | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="dreepy" /> |
| <a href="../pokemon-lookup.html?q=paldean-wooper">Paldean Wooper</a> | — | 5% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="paldean-wooper" /> |
| <a href="../pokemon-lookup.html?q=salandit">Salandit</a> | — | 20% | — | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="salandit" /> |
| <a href="../pokemon-lookup.html?q=dratini">Dratini</a> | — | — | 5% | — | — | 5% | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="dratini" /> |
| <a href="../pokemon-lookup.html?q=remoraid">Remoraid</a> | — | — | 5% | — | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="remoraid" /> |
| <a href="../pokemon-lookup.html?q=tentacool">Tentacool</a> | — | — | 60% | 30% | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="tentacool" /> |
| <a href="../pokemon-lookup.html?q=wailmer">Wailmer</a> | — | — | 30% | — | — | 40% | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="wailmer" /> |
| <a href="../pokemon-lookup.html?q=magikarp">Magikarp</a> | — | — | — | 70% | — | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="magikarp" /> |
| <a href="../pokemon-lookup.html?q=clamperl">Clamperl</a> | — | — | — | — | 20% | 40% | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="clamperl" /> |
| <a href="../pokemon-lookup.html?q=finneon">Finneon</a> | — | — | — | — | 20% | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="finneon" /> |
| <a href="../pokemon-lookup.html?q=psyduck">Psyduck</a> | — | — | — | — | 60% | — | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="psyduck" /> |
| <a href="../pokemon-lookup.html?q=chinchou">Chinchou</a> | — | — | — | — | — | 15% | <input type="checkbox" class="encounter-check" data-area="fresco-isles" data-species="chinchou" /> |

<script>
(function() {
  const STORAGE_KEY = 'lazarusEncounterChecks';
  function loadState() {
    try {
      return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}');
    } catch (_) {
      return {};
    }
  }
  function saveState(state) {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(state));
    } catch (_) {}
  }
  const state = loadState();
  const checkboxes = document.querySelectorAll('.encounter-check');
  checkboxes.forEach(cb => {
    const key = `${cb.dataset.area}|${cb.dataset.species}`;
    cb.checked = !!state[key];
    cb.addEventListener('change', () => {
      if (cb.checked) {
        state[key] = true;
      } else {
        delete state[key];
      }
      saveState(state);
    });
  });
})();
</script>

